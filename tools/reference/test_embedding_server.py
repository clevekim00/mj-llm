import json
import math
import threading
import unittest
from http.server import ThreadingHTTPServer
from urllib.error import HTTPError
from urllib.request import Request, urlopen
from unittest.mock import Mock, patch

from embedding_server import EmbeddingEngine, MODEL, RequestError, load_model, make_handler, prepare_request


class EmbeddingTests(unittest.TestCase):
    def test_search_and_titled_document_are_asymmetric(self):
        query, _, _ = prepare_request({"model": MODEL, "input": "검색", "mj": {"task": "code_retrieval"}})
        document, _, _ = prepare_request({"model": MODEL, "input": ["코드"],
                                         "mj": {"titles": ["main.rs"]}})
        self.assertEqual(query, ["task: code retrieval | query: 검색"])
        self.assertEqual(document, ["title: main.rs | text: 코드"])
        self.assertEqual(prepare_request({"model": MODEL, "input": "내용"})[0], ["title: none | text: 내용"])

    def test_invalid_requests(self):
        for changes in [{"input": []}, {"input": [1]}, {"input": " "}, {"input": ["a"] * 33},
                        {"dimensions": 300}, {"dimensions": True}, {"dimensions": 128.0},
                        {"encoding_format": "base64"}, {"model": "other"},
                        {"mj": {"task": []}}, {"mj": {"titles": []}},
                        {"mj": {"task": "search_query", "titles": ["title"]}},
                        {"mj": {"titles": [None]}}, {"image": "remote-url"}]:
            with self.subTest(changes=changes), self.assertRaises(RequestError):
                prepare_request({"model": MODEL, "input": "text", **changes})

    def test_encode_normalizes_after_truncation_and_disables_automatic_prompt(self):
        model = Mock()
        model.tokenizer.return_value = {"input_ids": [1, 2, 3]}
        model.encode.return_value.tolist.return_value = [[1 / math.sqrt(128)] * 128]
        result = EmbeddingEngine(model).embed({"model": MODEL, "input": "hello", "dimensions": 128})
        self.assertEqual(result["usage"]["total_tokens"], 3)
        self.assertEqual(len(result["data"][0]["embedding"]), 128)
        self.assertEqual(model.encode.call_args.kwargs["truncate_dim"], 128)
        self.assertTrue(model.encode.call_args.kwargs["normalize_embeddings"])
        self.assertEqual(model.encode.call_args.kwargs["prompt"], "")

    def test_overlong_input_is_rejected_without_silent_truncation(self):
        model = Mock()
        model.tokenizer.return_value = {"input_ids": [0] * 8193}
        with self.assertRaises(RequestError) as error:
            EmbeddingEngine(model).embed({"model": MODEL, "input": "long"})
        self.assertEqual(error.exception.code, "context_length_exceeded")
        model.encode.assert_not_called()

    def test_bad_vectors_and_busy_engine(self):
        model = Mock()
        model.tokenizer.return_value = {"input_ids": [0]}
        engine = EmbeddingEngine(model)
        for vector in [[float("nan")] * 128, [0.0] * 128, [1.0]]:
            model.encode.return_value.tolist.return_value = [vector]
            with self.assertRaises(RequestError):
                engine.embed({"model": MODEL, "input": "test", "dimensions": 128})
        with engine.lock, self.assertRaises(RequestError) as error:
            engine.embed({"model": MODEL, "input": "test"})
        self.assertEqual(error.exception.status, 429)

    def test_loader_omits_media_encoders_and_never_uses_fp16(self):
        torch, st = Mock(), Mock()
        torch.cuda.is_bf16_supported.return_value = True
        with patch.dict("sys.modules", {"torch": torch, "sentence_transformers": st}):
            load_model("cpu", True)
            kwargs = st.SentenceTransformer.call_args.kwargs
            self.assertEqual(kwargs["config_kwargs"], {"vision_config": None, "audio_config": None})
            self.assertEqual(kwargs["model_kwargs"]["torch_dtype"], torch.float32)
            self.assertTrue(kwargs["local_files_only"])
            load_model("cuda")
            self.assertEqual(st.SentenceTransformer.call_args.kwargs["model_kwargs"]["torch_dtype"], torch.bfloat16)

    def test_http_auth_and_error_envelopes(self):
        engine = Mock()
        engine.embed.side_effect = RequestError("Too long", 413, "context_length_exceeded")
        server = ThreadingHTTPServer(("127.0.0.1", 0), make_handler(engine, "test-token"))
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        root = f"http://127.0.0.1:{server.server_port}"
        try:
            with self.assertRaises(HTTPError) as error:
                urlopen(root + "/health", timeout=2)
            self.assertEqual(error.exception.code, 401)
            headers = {"Authorization": "Bearer test-token", "Content-Type": "application/json"}
            with urlopen(Request(root + "/health", headers=headers), timeout=2) as response:
                self.assertEqual(json.load(response), {"model": MODEL, "ready": True})
            with self.assertRaises(HTTPError) as error:
                urlopen(Request(root + "/v1/embeddings", data=b"{}", headers=headers), timeout=2)
            self.assertEqual(error.exception.code, 413)
            self.assertEqual(json.load(error.exception)["error"]["code"], "context_length_exceeded")
        finally:
            server.shutdown()
            server.server_close()
            thread.join()


if __name__ == "__main__":
    unittest.main()
