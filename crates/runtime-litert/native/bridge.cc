// Narrow synchronous C ABI. SDK objects never cross into Rust ownership.
#include <cstdint>
#include <cstring>
#include <memory>
#include "embedding_engine.h"

template<class T, void (*Free)(T*)> using Owned = std::unique_ptr<T, decltype(Free)>;
struct MjEmbedding {
  Owned<LiteRtLmEmbeddingEngine, litert_lm_embedding_engine_delete> engine{nullptr, litert_lm_embedding_engine_delete};
  Owned<LiteRtLmEmbeddingOptions, litert_lm_embedding_options_delete> options{nullptr, litert_lm_embedding_options_delete};
};

extern "C" {
// 0=ok, 1=invalid argument, 2=SDK failure, 3=C++ exception, 4=bad output.
int32_t mj_embedding_open(const char* model, const char* cache, MjEmbedding** out) noexcept {
  if (!model || !cache || !out) return 1;
  *out = nullptr;
  try {
    Owned<LiteRtLmEmbeddingEngineSettings, litert_lm_embedding_engine_settings_delete> settings(
        litert_lm_embedding_engine_settings_create(model, "cpu", "cpu", nullptr),
        litert_lm_embedding_engine_settings_delete);
    if (!settings) return 2;
    litert_lm_embedding_engine_settings_set_num_threads(settings.get(), 2);
    litert_lm_embedding_engine_settings_set_activation_data_type(settings.get(), kLiteRtLmActivationDataTypeFloat32);
    litert_lm_embedding_engine_settings_set_cache_dir(settings.get(), cache);
    litert_lm_embedding_engine_settings_set_min_input_length(settings.get(), 128);
    litert_lm_embedding_engine_settings_set_max_input_length(settings.get(), 512);
    litert_lm_embedding_engine_settings_set_vision_tokens_per_image(settings.get(), 70);
    auto handle = std::make_unique<MjEmbedding>();
    handle->engine.reset(litert_lm_embedding_engine_create(settings.get()));
    if (!handle->engine) return 2;
    handle->options.reset(litert_lm_embedding_options_create());
    if (!handle->options) return 2;
    litert_lm_embedding_options_set_normalize(handle->options.get(), true);
    litert_lm_embedding_options_set_insert_special_tokens(handle->options.get(), true);
    litert_lm_embedding_options_set_input_overflow_strategy(handle->options.get(), kLiteRtLmInputOverflowStrategyError);
    litert_lm_embedding_options_set_output_size(handle->options.get(), 768);
    litert_lm_embedding_options_set_vision_tokens_per_image(handle->options.get(), 70);
    *out = handle.release();
    return 0;
  } catch (...) { return 3; }
}

int32_t mj_embedding_compute(MjEmbedding* handle, uint32_t kind, const uint8_t* data,
    size_t size, float* output, size_t capacity, size_t* written) noexcept {
  if (!handle || !data || !size || !output || !written || kind > 1) return 1;
  *written = 0;
  try {
    Owned<LiteRtLmInputData, litert_lm_input_data_delete> input(
        litert_lm_input_data_create(kind == 0 ? kLiteRtLmInputDataTypeText : kLiteRtLmInputDataTypeImage, data, size),
        litert_lm_input_data_delete);
    if (!input) return 2;
    const LiteRtLmInputData* inputs[] = {input.get()};
    Owned<LiteRtLmEmbeddingResponse, litert_lm_embedding_response_delete> response(
        litert_lm_embedding_engine_compute_embedding(handle->engine.get(), inputs, 1, handle->options.get()),
        litert_lm_embedding_response_delete);
    if (!response) return 2;
    const size_t count = litert_lm_embedding_response_get_size(response.get());
    const float* values = litert_lm_embedding_response_get_values(response.get());
    if (!values || count != 768 || count > capacity) return 4;
    std::memcpy(output, values, count * sizeof(float));
    *written = count;
    return 0;
  } catch (...) { return 3; }
}

void mj_embedding_close(MjEmbedding* handle) noexcept {
  // No in-flight calls: safe Rust requires exclusive mutable access and is !Send/!Sync.
  try { delete handle; } catch (...) { /* Never unwind through the C ABI. */ }
}
}

#include "conversation.h"

struct MjGenerator {
  Owned<LiteRtLmEngine, litert_lm_engine_delete> engine{nullptr, litert_lm_engine_delete};
};
extern "C" {
int32_t mj_generation_open(const char* model, const char* cache, MjGenerator** out) noexcept {
  if (!model || !cache || !out) return 1;
  *out = nullptr;
  try {
    Owned<LiteRtLmEngineSettings, litert_lm_engine_settings_delete> settings(
        litert_lm_engine_settings_create(model, "cpu", nullptr, nullptr), litert_lm_engine_settings_delete);
    if (!settings) return 2;
    litert_lm_engine_settings_set_num_threads(settings.get(), 2);
    litert_lm_engine_settings_set_max_num_tokens(settings.get(), 512);
    litert_lm_engine_settings_set_cache_dir(settings.get(), cache);
    auto handle = std::make_unique<MjGenerator>();
    handle->engine.reset(litert_lm_engine_create(settings.get()));
    if (!handle->engine) return 2;
    *out = handle.release();
    return 0;
  } catch (...) { return 3; }
}
int32_t mj_generation_run(MjGenerator* handle, const char* message, uint8_t* output,
    size_t capacity, size_t* written) noexcept {
  if (!handle || !message || !output || !written) return 1;
  *written = 0;
  try {
    Owned<LiteRtLmSamplerParams, litert_lm_sampler_params_delete> sampler(
        litert_lm_sampler_params_create(kLiteRtLmSamplerTypeTopP),litert_lm_sampler_params_delete);
    Owned<LiteRtLmSessionConfig, litert_lm_session_config_delete> session(
        litert_lm_session_config_create(),litert_lm_session_config_delete);
    Owned<LiteRtLmConversationConfig, litert_lm_conversation_config_delete> config(
        litert_lm_conversation_config_create(),litert_lm_conversation_config_delete);
    Owned<LiteRtLmConversationOptionalArgs, litert_lm_conversation_optional_args_delete> options(
        litert_lm_conversation_optional_args_create(),litert_lm_conversation_optional_args_delete);
    if (!sampler || !session || !config || !options) return 2;
    litert_lm_sampler_params_set_top_k(sampler.get(),1);
    litert_lm_sampler_params_set_top_p(sampler.get(),1.0f);
    litert_lm_sampler_params_set_temperature(sampler.get(),1.0f);
    litert_lm_sampler_params_set_seed(sampler.get(),0);
    litert_lm_session_config_set_sampler_params(session.get(),sampler.get());
    litert_lm_session_config_set_max_output_tokens(session.get(),32);
    litert_lm_conversation_config_set_session_config(config.get(),session.get());
    litert_lm_conversation_config_set_extra_context(config.get(),"{\"enable_thinking\":false}");
    litert_lm_conversation_optional_args_set_max_output_tokens(options.get(),32);
    Owned<LiteRtLmConversation,litert_lm_conversation_delete> conversation(
        litert_lm_conversation_create(handle->engine.get(),config.get()),litert_lm_conversation_delete);
    if (!conversation) return 2;
    Owned<LiteRtLmJsonResponse,litert_lm_json_response_delete> response(
        litert_lm_conversation_send_message(conversation.get(),message,"{\"enable_thinking\":false}",options.get()),
        litert_lm_json_response_delete);
    if (!response) return 2;
    const char* json = litert_lm_json_response_get_string(response.get());
    if (!json) return 4;
    const size_t size = std::strlen(json);
    if (size == 0 || size > capacity) return 4;
    std::memcpy(output,json,size);
    *written = size;
    return 0;
  } catch (...) { return 3; }
}
void mj_generation_close(MjGenerator* handle) noexcept {
  try { delete handle; } catch (...) {}
}
}
