#!/usr/bin/env python3
"""Opt-in, authenticated loopback service for EmbeddingGemma 2 text embeddings."""
import argparse
import hmac
import json
import math
import os
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

MODEL = "google/embeddinggemma-2"
DIMENSIONS = (128, 256, 512, 768)
TASKS = {
    "search_query": "task: search result | query: ",
    "question_answering": "task: question answering | query: ",
    "code_retrieval": "task: code retrieval | query: ",
    "fact_checking": "task: fact checking | query: ",
    "classification": "task: classification | query: ",
    "clustering": "task: clustering | query: ",
    "sentence_similarity": "task: sentence similarity | query: ",
}
MAX_BODY = 1_048_576


class RequestError(Exception):
    def __init__(self, message, status=400, code="invalid_request"):
        super().__init__(message)
        self.status, self.code = status, code


def prepare_request(body):
    if not isinstance(body, dict):
        raise RequestError("Expected a JSON object")
    if set(body) - {"model", "input", "dimensions", "encoding_format", "user", "mj"}:
        raise RequestError("Unknown request field")
    if body.get("model") != MODEL:
        raise RequestError(f"model must be {MODEL}")
    if body.get("encoding_format", "float") != "float":
        raise RequestError("Only encoding_format=float is supported")
    if "user" in body and not isinstance(body["user"], str):
        raise RequestError("user must be a string")
    dimensions = body.get("dimensions", 768)
    if type(dimensions) is not int or dimensions not in DIMENSIONS:
        raise RequestError("dimensions must be 128, 256, 512 or 768")
    texts = body.get("input")
    if isinstance(texts, str):
        texts = [texts]
    if not isinstance(texts, list) or not 1 <= len(texts) <= 32:
        raise RequestError("input must be a string or 1–32 strings")
    if any(not isinstance(text, str) or not text.strip() for text in texts):
        raise RequestError("Each input must be a nonempty text string; media and token IDs are unsupported")
    options = body.get("mj", {})
    if not isinstance(options, dict) or set(options) - {"task", "titles"}:
        raise RequestError("mj accepts task and titles only")
    task = options.get("task", "document")
    if not isinstance(task, str) or task not in {*TASKS, "document"}:
        raise RequestError("Unsupported mj.task")
    titles = options.get("titles")
    if titles is not None:
        if task != "document" or not isinstance(titles, list) or len(titles) != len(texts):
            raise RequestError("mj.titles requires document task and one title per input")
        if any(not isinstance(title, str) or not title.strip() for title in titles):
            raise RequestError("Each title must be a nonempty string")
    if task == "document":
        titles = titles or ["none"] * len(texts)
        formatted = [f"title: {title} | text: {text}" for title, text in zip(titles, texts)]
    else:
        formatted = [TASKS[task] + text for text in texts]
    return formatted, dimensions, task


class EmbeddingEngine:
    def __init__(self, model):
        self.model = model
        # One inference at a time bounds peak RAM; concurrent requests fail fast.
        self.lock = threading.Lock()

    def embed(self, body):
        texts, dimensions, task = prepare_request(body)
        if not self.lock.acquire(blocking=False):
            raise RequestError("Embedding service is busy; retry later", 429, "rate_limited")
        try:
            counts = [len(self.model.tokenizer(text, truncation=False)["input_ids"]) for text in texts]
            if any(count > 8192 for count in counts):
                raise RequestError("Input including task prefix exceeds 8192 tokens; split it into chunks",
                                   413, "context_length_exceeded")
            vectors = self.model.encode(
                texts, prompt="", batch_size=1, truncate_dim=dimensions,
                normalize_embeddings=True, convert_to_numpy=True, show_progress_bar=False,
            ).tolist()
            if len(vectors) != len(texts) or any(
                len(vector) != dimensions or not all(math.isfinite(v) for v in vector)
                or abs(sum(v*v for v in vector) - 1.0) > 0.01 for vector in vectors
            ):
                raise RequestError("Model returned invalid embeddings", 502, "upstream_error")
            return {
                "object": "list", "model": MODEL,
                "data": [{"object": "embedding", "index": i, "embedding": vector}
                         for i, vector in enumerate(vectors)],
                "usage": {"prompt_tokens": sum(counts), "total_tokens": sum(counts)},
                "mj": {"provider": "sentence-transformers", "task": task,
                       "dimensions": dimensions, "normalized": True, "modalities": ["text"]},
            }
        finally:
            self.lock.release()


def make_handler(engine, token):
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_args):
            pass  # Do not record user input or authentication tokens.

        def respond(self, status, body):
            payload = json.dumps(body, allow_nan=False).encode()
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)

        def authorized(self):
            expected = f"Bearer {token}".encode()
            supplied = self.headers.get("Authorization", "").encode()
            if hmac.compare_digest(expected, supplied):
                return True
            self.respond(401, {"error": {"message": "Authentication required",
                                       "type": "mj_gateway_error", "code": "invalid_request"}})
            return False

        def do_GET(self):
            if self.authorized():
                if self.path == "/health":
                    self.respond(200, {"ready": True, "model": MODEL})
                else:
                    self.respond(404, {"error": "Not found"})

        def do_POST(self):
            if not self.authorized():
                return
            try:
                if self.path != "/v1/embeddings":
                    raise RequestError("Not found", 404)
                if self.headers.get("Transfer-Encoding"):
                    raise RequestError("Chunked requests are unsupported")
                length = int(self.headers.get("Content-Length", "0"))
                if not 0 < length <= MAX_BODY:
                    raise RequestError("Request body must be between 1 byte and 1 MiB")
                self.connection.settimeout(10)
                body = json.loads(self.rfile.read(length))
                result = engine.embed(body)
                self.respond(200, result)
            except (ValueError, UnicodeError):
                self.respond(400, {"error": {"message": "Invalid JSON request",
                                           "type": "mj_gateway_error", "code": "invalid_request"}})
            except RequestError as error:
                self.respond(error.status, {"error": {"message": str(error),
                    "type": "mj_gateway_error", "code": error.code}})
            except Exception:
                self.respond(502, {"error": {"message": "Embedding inference failed",
                                           "type": "mj_gateway_error", "code": "upstream_error"}})
    return Handler


def load_model(device, offline=False):
    import torch
    from sentence_transformers import SentenceTransformer
    dtype = torch.bfloat16 if device == "cuda" and torch.cuda.is_bf16_supported() else torch.float32
    return SentenceTransformer(
        MODEL, device=device, local_files_only=offline,
        config_kwargs={"vision_config": None, "audio_config": None},
        model_kwargs={"torch_dtype": dtype},
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--port", type=int, default=3211)
    parser.add_argument("--device", choices=["cpu", "mps", "cuda"], default="cpu")
    parser.add_argument("--offline", action="store_true", help="Load cached model files only")
    args = parser.parse_args()
    token = os.environ.get("MJ_HUB_TOKEN", "")
    if len(token) < 32:
        parser.error("Set the same MJ_HUB_TOKEN (at least 32 characters) for the hub and this service")
    engine = EmbeddingEngine(load_model(args.device, args.offline))
    server = ThreadingHTTPServer(("127.0.0.1", args.port), make_handler(engine, token))
    print(f"EmbeddingGemma 2 text service ready at http://127.0.0.1:{args.port}", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
