# EXAONE Deep
<br>
<p align="center">
<img src="assets/EXAONE_Symbol+BI_3d.png", width="400", style="margin: 40 auto;">
<br>
<p align="center"> 🤗 <a href="https://huggingface.co/collections/LGAI-EXAONE/exaone-deep-67d119918816ec6efa79a4aa">Hugging Face</a> &nbsp | &nbsp 📝 <a href="https://www.lgresearch.ai/news/view?seq=543"> Blog</a> &nbsp | &nbsp 📑 <a href="https://arxiv.org/abs/2503.12524"> Documentation </a>
<br>

<br>

## Introduction

We introduce EXAONE Deep, which exhibits superior capabilities in various reasoning tasks including math and coding benchmarks, ranging from 2.4B to 32B parameters developed and released by LG AI Research. Evaluation results show that 1) EXAONE Deep 2.4B outperforms other models of comparable size, 2) EXAONE Deep 7.8B outperforms not only open-weight models of comparable scale but also a proprietary reasoning model OpenAI o1-mini, and 3) EXAONE Deep 32B demonstrates competitive performance against leading open-weight models.

<br>
<p align="center">
<img src="assets/exaone_deep_overall_performance.png", width="90%", style="margin: 40 auto;">
<br>

Our documentation consists of the following sections:

- [Performance](#performance): Experimental results of EXAONE Deep models.
- [Quickstart](#quickstart): A basic guide to using EXAONE Deep models with Transformers.
- [Quantized Models](#quantized-models): An explanation of quantized EXAONE Deep weights in `AWQ` and `GGUF` format.
- [Run Locally](#run-locally): A guide to running EXAONE Deep models locally with `llama.cpp` and `Ollama` frameworks.
- [Deployment](#deployment): A guide to running EXAONE Deep models with `TensorRT-LLM`, `vLLM`, and `SGLang` deployment frameworks.
- [Usage Guideline](#usage-guideline): A guide to utilizing EXAONE Deep models to achieve the expected performance.

<br>

## News

- 2025.03.18: We release the EXAONE Deep, reasoning enhanced language models, including 2.4B, 7.8B, and 32B. Check out the 📑 [Documentation](https://arxiv.org/abs/2503.12524)!

<br>

## Performance

Some experimental results are shown below. The full evaluation results can be found in the [Documentation](https://arxiv.org/abs/2503.12524).

<br>

<table>
    <tr>
        <th>Models</th>
        <th>MATH-500 (pass@1)</th>
        <th>AIME 2024 (pass@1 / cons@64)</th>
        <th>AIME 2025 (pass@1 / cons@64)</th>
        <th>CSAT Math 2025 (pass@1)</th>
        <th>GPQA Diamond (pass@1)</th>
        <th>Live Code Bench (pass@1)</th>
    </tr>
    <tr>
        <td>EXAONE Deep 32B</td>
        <td>95.7</td>
        <td>72.1 / <strong>90.0</strong></td>
        <td>65.8 / <strong>80.0</strong></td>
        <td><strong>94.5</strong></td>
        <td>66.1</td>
        <td>59.5</td>
    </tr>
    <tr>
        <td>DeepSeek-R1-Distill-Qwen-32B</td>
        <td>94.3</td>
        <td>72.6 / 83.3</td>
        <td>55.2 / 73.3</td>
        <td>84.1</td>
        <td>62.1</td>
        <td>57.2</td>
    </tr>
    <tr>
        <td>QwQ-32B</td>
        <td>95.5</td>
        <td>79.5 / 86.7</td>
        <td><strong>67.1</strong> / 76.7</td>
        <td>94.4</td>
        <td>63.3</td>
        <td>63.4</td>
    </tr>
    <tr>
        <td>DeepSeek-R1-Distill-Llama-70B</td>
        <td>94.5</td>
        <td>70.0 / 86.7</td>
        <td>53.9 / 66.7</td>
        <td>88.8</td>
        <td>65.2</td>
        <td>57.5</td>
    </tr>
    <tr>
        <td>DeepSeek-R1 (671B)</td>
        <td><strong>97.3</strong></td>
        <td><strong>79.8</strong> / 86.7</td>
        <td>66.8 / <strong>80.0</strong></td>
        <td>89.9</td>
        <td><strong>71.5</strong></td>
        <td><strong>65.9</strong></td>
    </tr>
    <tr>
        <th colspan="7" height="30px"></th>
    </tr>
    <tr>
        <td>EXAONE Deep 7.8B</td>
        <td><strong>94.8</strong></td>
        <td><strong>70.0</strong> / <strong>83.3</strong></td>
        <td><strong>59.6</strong> / <strong>76.7</strong></td>
        <td><strong>89.9</strong></td>
        <td><strong>62.6</strong></td>
        <td><strong>55.2</strong></td>
    </tr>
    <tr>
        <td>DeepSeek-R1-Distill-Qwen-7B</td>
        <td>92.8</td>
        <td>55.5 / <strong>83.3</strong></td>
        <td>38.5 / 56.7</td>
        <td>79.7</td>
        <td>49.1</td>
        <td>37.6</td>
    </tr>
    <tr>
        <td>DeepSeek-R1-Distill-Llama-8B</td>
        <td>89.1</td>
        <td>50.4 / 80.0</td>
        <td>33.6 / 53.3</td>
        <td>74.1</td>
        <td>49.0</td>
        <td>39.6</td>
    </tr>
    <tr>
        <td>OpenAI o1-mini</td>
        <td>90.0</td>
        <td>63.6 / 80.0</td>
        <td>54.8 / 66.7</td>
        <td>84.4</td>
        <td>60.0</td>
        <td>53.8</td>
    </tr>
    <tr>
        <th colspan="7" height="30px"></th>
    </tr>
    <tr>
        <td>EXAONE Deep 2.4B</td>
        <td><strong>92.3</strong></td>
        <td><strong>52.5</strong> / <strong>76.7</strong></td>
        <td><strong>47.9</strong> / <strong>73.3</strong></td>
        <td><strong>79.2</strong></td>
        <td><strong>54.3</strong></td>
        <td><strong>46.6</strong></td>
    </tr>
    <tr>
        <td>DeepSeek-R1-Distill-Qwen-1.5B</td>
        <td>83.9</td>
        <td>28.9 / 52.7</td>
        <td>23.9 / 36.7</td>
        <td>65.6</td>
        <td>33.8</td>
        <td>16.9</td>
    </tr>
</table>

<br>

## Quickstart

- You need to install `transformers>=4.43.1` for the EXAONE Deep models. The latest version is recommended to use.

Here is the example code to show how to use EXAONE Deep models.

> [!Tip]
> In all the examples below, you can use another size model by changing 7.8B to 32B or 2.4B.

```python
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer, TextIteratorStreamer
from threading import Thread

model_name = "LGAI-EXAONE/EXAONE-Deep-7.8B"
streaming = True    # choose the streaming option

model = AutoModelForCausalLM.from_pretrained(
    model_name,
    torch_dtype=torch.bfloat16,
    trust_remote_code=True,
    device_map="auto"
)
tokenizer = AutoTokenizer.from_pretrained(model_name)

# Choose your prompt:
#   Math example (AIME 2024)
prompt = r"""Let $x,y$ and $z$ be positive real numbers that satisfy the following system of equations:
\[\log_2\left({x \over yz}\right) = {1 \over 2}\]\[\log_2\left({y \over xz}\right) = {1 \over 3}\]\[\log_2\left({z \over xy}\right) = {1 \over 4}\]
Then the value of $\left|\log_2(x^4y^3z^2)\right|$ is $\tfrac{m}{n}$ where $m$ and $n$ are relatively prime positive integers. Find $m+n$.

Please reason step by step, and put your final answer within \boxed{}."""
#   Korean MCQA example (CSAT Math 2025)
prompt = r"""Question : $a_1 = 2$인 수열 $\{a_n\}$과 $b_1 = 2$인 등차수열 $\{b_n\}$이 모든 자연수 $n$에 대하여\[\sum_{k=1}^{n} \frac{a_k}{b_{k+1}} = \frac{1}{2} n^2\]을 만족시킬 때, $\sum_{k=1}^{5} a_k$의 값을 구하여라.

Options :
A) 120
B) 125
C) 130
D) 135
E) 140
 
Please reason step by step, and you should write the correct option alphabet (A, B, C, D or E) within \\boxed{}."""

messages = [
    {"role": "user", "content": prompt}
]
input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt"
)

if streaming:
    streamer = TextIteratorStreamer(tokenizer)
    thread = Thread(target=model.generate, kwargs=dict(
        input_ids=input_ids.to("cuda"),
        eos_token_id=tokenizer.eos_token_id,
        max_new_tokens=32768,
        do_sample=True,
        temperature=0.6,
        top_p=0.95,
        streamer=streamer
    ))
    thread.start()

    for text in streamer:
        print(text, end="", flush=True)
else:
    output = model.generate(
        input_ids.to("cuda"),
        eos_token_id=tokenizer.eos_token_id,
        max_new_tokens=32768,
        do_sample=True,
        temperature=0.6,
        top_p=0.95,
    )
    print(tokenizer.decode(output[0]))
```

> [!Important]
> The EXAONE Deep models are trained with an optimized configuration,
> so we recommend following the [Usage Guideline](#usage-guideline) section to achieve optimal performance.

<br>

## Quantized Models

We introduce a series of quantized weights of EXAONE Deep models.

### AWQ

We provide AWQ-quantized weights of EXAONE Deep models, quantized using `AutoAWQ` library. Please refer to the [EXAONE Deep collection](https://huggingface.co/collections/LGAI-EXAONE/exaone-deep-67d119918816ec6efa79a4aa) for pre-quantized weights, and the [AutoAWQ documentation](https://github.com/casper-hansen/AutoAWQ) for more details.

You need to install the latest version of AutoAWQ library (`autoawq>=0.2.8`) to load the AWQ-quantized version of EXAONE Deep models.

```bash
pip install autoawq
```

You can load the model in similar ways to the original models, only changing the model name. It automatically loads with AWQ configuration of the model. Please check the [Quickstart section](#quickstart) above for more details.

### GGUF

We provide weights in `BF16` format and quantized weights in `Q8_0`, `Q6_K`, `Q5_K_M`, `Q4_K_M`, `IQ4_XS`. 

The example below is for the 7.8B model in BF16 format. Please refer to the [EXAONE Deep collection](https://huggingface.co/collections/LGAI-EXAONE/exaone-deep-67d119918816ec6efa79a4aa) to find quantized models. You may need to install `huggingface_hub` to download the GGUF weights.

```bash
# (optional) install huggingface_hub
pip install huggingface_hub

# Download the GGUF weights
huggingface-cli download LGAI-EXAONE/EXAONE-Deep-7.8B-GGUF \
    --include "EXAONE-Deep-7.8B-BF16*.gguf" \
    --local-dir .
```

## Run Locally

For end users, we introduce two ways to run EXAONE Deep models locally. 

> [!Note]
> We highly recommend to use repetition penalty not exceeding 1.0 for better generation quality.

### llama.cpp

You can run EXAONE models with llama.cpp as follows:

1. Install llama.cpp. Please refer to the [llama.cpp repository](https://github.com/ggerganov/llama.cpp) for more details.

2. Download EXAONE Deep model in GGUF format.

```bash
huggingface-cli download LGAI-EXAONE/EXAONE-Deep-7.8B-GGUF \
    --include "EXAONE-Deep-7.8B-BF16*.gguf" \
    --local-dir .
```

3. Run the model with llama.cpp in conversational mode. We set chat-template explicitly to handle reasoning steps properly. 

```bash
llama-cli -m ./EXAONE-Deep-7.8B-BF16.gguf \
    -sys "" \
    -c 32768 \
    --temp 0.6 \
    --top-p 0.95 \
    --jinja \
    --chat-template "{% for message in messages %}{% if loop.first and message['role'] != 'system' %}{{ '[|system|][|endofturn|]\n' }}{% endif %}{% set content = message['content'] %}{% if '</thought>' in content %}{% set content = content.split('</thought>')[-1].lstrip('\\n') %}{% endif %}{{ '[|' + message['role'] + '|]' + content }}{% if not message['role'] == 'user' %}{{ '[|endofturn|]' }}{% endif %}{% if not loop.last %}{{ '\n' }}{% endif %}{% endfor %}{% if add_generation_prompt %}{{ '\n[|assistant|]<thought>\n' }}{% endif %}"
```

- In case of using EXAONE Deep 32B model with BF16 precision, you may need to download all split files and merge them before running the model.

    ```bash
    # Download all split files
    huggingface-cli download LGAI-EXAONE/EXAONE-Deep-32B-GGUF \
        --include "EXAONE-Deep-32B-BF16*.gguf" \
        --local-dir .

    # Merge all split files
    llama-gguf-split --merge \
        ./EXAONE-Deep-32B-BF16-00001-of-00002.gguf \
        ./EXAONE-Deep-32B-BF16.gguf
    ```

### Ollama

EXAONE Deep models are uploaded to Ollama model library. You can easily use EXAONE models as follows:

1. Install Ollama. Please refer to the [Ollama repository](https://github.com/ollama/ollama) for more details.

2. Run EXAONE Deep model as follows:
```bash
ollama run exaone-deep:7.8b
```

> [!Note]
> In above example, the model `exaone-deep:7.8b` is quantized in `Q4_K_M`. If you would like to know a list of available models, 
> please refer to the [EXAONE Deep Ollama page](https://ollama.com/library/exaone-deep) for more details.

Or, you can create and run EXAONE Deep models with GGUF format for customizing.

1. Install Ollama. Please refer to 

<!-- truncated for clean-room docs-read -->
