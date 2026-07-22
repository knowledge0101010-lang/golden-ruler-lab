# EXAONE 3.5
<br>
<p align="center">
<img src="assets/EXAONE_Symbol+BI_3d.png", width="400", style="margin: 40 auto;">
<br>
<p align="center"> 🤗 <a href="https://huggingface.co/collections/LGAI-EXAONE/exaone-35-674d0e1bb3dcd2ab6f39dbb4">Hugging Face</a> &nbsp | &nbsp 📝 <a href="https://www.lgresearch.ai/blog/view?seq=507"> Blog</a> &nbsp | &nbsp 📑 <a href="https://arxiv.org/abs/2412.04862"> Technical Report </a>
<!-- TODO: Add Demo page? -->
<br>

<br>

## Introduction

We introduce EXAONE 3.5, a collection of instruction-tuned bilingual (English and Korean) generative models ranging from 2.4B to 32B parameters, developed and released by LG AI Research. EXAONE 3.5 language models include: 1) 2.4B model optimized for deployment on small or resource-constrained devices, 2) 7.8B model matching the size of its predecessor but offering improved performance, and 3) 32B model delivering powerful performance. All models support long-context processing of up to 32K tokens. Each model demonstrates state-of-the-art performance in real-world use cases and long-context understanding, while remaining competitive in general domains compared to recently released models of similar sizes.

Our documentation consists of the following sections:

- [Performance](#performance): Experimental results of EXAONE 3.5 models.
- [Quickstart](#quickstart): A basic guide to using EXAONE 3.5 models with Transformers.
- [Quantized Models](#quantized-models): An explanation of quantized EXAONE 3.5 weights in `AWQ` and `GGUF` format.
- [Run Locally](#run-locally): A guide to running EXAONE 3.5 models locally with `llama.cpp` and `Ollama` frameworks.
- [Deployment](#deployment): A guide to running EXAONE 3.5 models with `TensorRT-LLM`, `vLLM`, and `SGLang` deployment frameworks.

<br>

## News

- 2024.12.11: EXAONE 3.5 is now avaiable on Ollama model library. 
<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;You can now install `AutoAWQ` library via pip without using the git repository.
- 2024.12.10: We update the EXAONE Modelfile for Ollama. Please use the new one.
- 2024.12.09: We release the EXAONE 3.5 language model series including 2.4B, 7.8B, and 32B instruction-tuned models. Check out the 📑 [Technical Report](https://arxiv.org/abs/2412.04862)!

<br>

## Performance

Some experimental results are shown below. The full evaluation results can be found in the [Technical Report](https://arxiv.org/abs/2412.04862).

<br>

<table>
    <tr>
        <th>Models</th>
        <th>MT-Bench</th>
        <th>LiveBench</th>
        <th>Arena-Hard</th>
        <th>AlpacaEval</th>
        <th>IFEval</th>
        <th>KoMT-Bench[1]</th>
        <th>LogicKor</th>
    </tr>
    <tr>
        <td>EXAONE 3.5 32B</td>
        <td align="center"><strong>8.51</strong></td>
        <td align="center">43.0</td>
        <td align="center"><strong>78.6</strong></td>
        <td align="center"><strong>60.6</strong></td>
        <td align="center"><strong>81.7</strong></td>
        <td align="center"><strong>8.05</strong></td>
        <td align="center"><strong>9.06</strong></td>
    </tr>
    <tr>
        <td>Qwen 2.5 32B</td>
        <td align="center">8.49</td>
        <td align="center"><strong>50.6</strong></td>
        <td align="center">67.0</td>
        <td align="center">41.0</td>
        <td align="center">78.7</td>
        <td align="center">7.75</td>
        <td align="center">8.89</td>
    </tr>
    <tr>
        <td>C4AI Command R 32B</td>
        <td align="center">7.38</td>
        <td align="center">29.7</td>
        <td align="center">17.0</td>
        <td align="center">25.9</td>
        <td align="center">26.1</td>
        <td align="center">6.72</td>
        <td align="center">8.24</td>
    </tr>
    <tr>
        <td>Gemma 2 27B</td>
        <td align="center">8.28</td>
        <td align="center">40.0</td>
        <td align="center">57.5</td>
        <td align="center">52.2</td>
        <td align="center">59.7</td>
        <td align="center">7.19</td>
        <td align="center">8.56</td>
    </tr>
    <tr>
        <td>Yi 1.5 34B</td>
        <td align="center">7.64</td>
        <td align="center">26.2</td>
        <td align="center">23.1</td>
        <td align="center">34.8</td>
        <td align="center">55.5</td>
        <td align="center">4.88</td>
        <td align="center">6.33</td>
    </tr>
    <tr>
        <th colspan="8" height="30px"></th>
    </tr>
    <tr>
        <td>EXAONE 3.5 7.8B</td>
        <td align="center"><strong>8.29</strong></td>
        <td align="center"><strong>39.8</strong></td>
        <td align="center"><strong>68.7</strong></td>
        <td align="center"><strong>54.2</strong></td>
        <td align="center"><strong>78.9</strong></td>
        <td align="center"><strong>7.96</strong></td>
        <td align="center"><strong>9.08</strong></td>
    </tr>
    <tr>
        <td>Qwen 2.5 7B</td>
        <td align="center">6.48</td>
        <td align="center">35.6</td>
        <td align="center">48.9</td>
        <td align="center">31.7</td>
        <td align="center">72.5</td>
        <td align="center">5.19</td>
        <td align="center">6.38</td>
    </tr>
    <tr>
        <td>Llama 3.1 8B</td>
        <td align="center">7.59</td>
        <td align="center">28.3</td>
        <td align="center">27.7</td>
        <td align="center">25.7</td>
        <td align="center">74.5</td>
        <td align="center">4.85</td>
        <td align="center">5.99</td>
    </tr>
    <tr>
        <td>Gemma 2 9B</td>
        <td align="center">7.64</td>
        <td align="center">32.1</td>
        <td align="center">43.6</td>
        <td align="center">47.3</td>
        <td align="center">54.7</td>
        <td align="center">7.10</td>
        <td align="center">8.05</td>
    </tr>
    <tr>
        <td>Phi 3 small (7B)</td>
        <td align="center">7.63</td>
        <td align="center">27.9</td>
        <td align="center">26.8</td>
        <td align="center">29.2</td>
        <td align="center">59.5</td>
        <td align="center">3.22</td>
        <td align="center">3.99</td>
    </tr>
    <tr>
        <th colspan="8" height="30px"></th>
    </tr>
    <tr>
        <td>EXAONE 3.5 2.4B</td>
        <td align="center"><strong>7.81</strong></td>
        <td align="center"><strong>33.0</strong></td>
        <td align="center"><strong>48.2</strong></td>
        <td align="center"><strong>37.1</strong></td>
        <td align="center"><strong>73.6</strong></td>
        <td align="center"><strong>7.24</strong></td>
        <td align="center"><strong>8.51</strong></td>
    </tr>
    <tr>
        <td>Qwen 2.5 3B</td>
        <td align="center">7.21</td>
        <td align="center">25.7</td>
        <td align="center">26.4</td>
        <td align="center">17.4</td>
        <td align="center">60.8</td>
        <td align="center">5.68</td>
        <td align="center">5.21</td>
    </tr>
    <tr>
        <td>Qwen 2.5 1.5B</td>
        <td align="center">5.72</td>
        <td align="center">19.2</td>
        <td align="center">10.6</td>
        <td align="center">8.4</td>
        <td align="center">40.7</td>
        <td align="center">3.87</td>
        <td align="center">3.60</td>
    </tr>
    <tr>
        <td>Llama 3.2 3B</td>
        <td align="center">6.94</td>
        <td align="center">24.0</td>
        <td align="center">14.2</td>
        <td align="center">18.7</td>
        <td align="center">70.1</td>
        <td align="center">3.16</td>
        <td align="center">2.86</td>
    </tr>
    <tr>
        <td>Gemma 2 2B</td>
        <td align="center">7.20</td>
        <td align="center">20.0</td>
        <td align="center">19.1</td>
        <td align="center">29.1</td>
        <td align="center">50.5</td>
        <td align="center">4.83</td>
        <td align="center">5.29</td>
    </tr>
</table>

- [1] KoMT-Bench is a dataset created by translating MT-Bench into Korean; see [README](https://github.com/LG-AI-EXAONE/KoMT-Bench) for more details.

<br>

## Quickstart

- You need to install `transformers>=4.43.0` for the EXAONE 3.5 models. The Latest version is recommended to use.

Here is the example code to show how to use EXAONE 3.5 models.

> [!Tip]
> In all examples below, you can use another size model by changing 7.8B to 32B or 2.4B.

```python
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer

model_name = "LGAI-EXAONE/EXAONE-3.5-7.8B-Instruct"

model = AutoModelForCausalLM.from_pretrained(
    model_name,
    torch_dtype=torch.bfloat16,
    trust_remote_code=True,
    device_map="auto"
)
tokenizer = AutoTokenizer.from_pretrained(model_name)

# Choose your prompt
prompt = "Explain how wonderful you are"  # English example
prompt = "스스로를 자랑해 봐"       # Korean example

messages = [
    {"role": "system", "content": "You are EXAONE model from LG AI Research, a helpful assistant."},
    {"role": "user", "content": prompt}
]
input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt"
)

output = model.generate(
    input_ids.to("cuda"),
    eos_token_id=tokenizer.eos_token_id,
    max_new_tokens=128,
    do_sample=False,
)
print(tokenizer.decode(output[0]))
```

> [!Note]
> The EXAONE 3.5 instruction-tuned language models were trained to utilize the system prompt,
> so we highly recommend using the system prompts provided in the code snippet above.

<br>

## Quantized Models

We introduce a series of quantized weights of EXAONE 3.5 models.

### AWQ

We provide AWQ-quantized weights of EXAONE 3.5 models, quantized using `AutoAWQ` library. Please refer to the [AutoAWQ documentation](https://github.com/casper-hansen/AutoAWQ) for more details.

You need to install the latest version of AutoAWQ library (`autoawq>=0.2.7.post3`) to load the AWQ-quantized version of EXAONE 3.5 models.

```bash
pip install autoawq
```

You can load the model in similar ways to the original models, only changing the model name. It automatically loads with AWQ configuration of the model. Please check the [Quickstart section](#quickstart) above for more details.

### GGUF

We provide weights in `BF16` format and quantized weights in `Q8_0`, `Q6_K`, `Q5_K_M`, `Q4_K_M`, `IQ4_XS`. 

The example below is for the 7.8B model in BF16 format. Please refer to the [EXAONE 3.5 collection](https://huggingface.co/collections/LGAI-EXAONE/exaone-35-674d0e1bb3dcd2ab6f39dbb4) to find quantized models. You may need to install `huggingface_hub` to download the GGUF weights.

```bash
# (optional) install huggingface_hub
pip install huggingface_hub

# Download the GGUF weights
huggingface-cli download LGAI-EXAONE/EXAONE-3.5-7.8B-Instruct-GGUF \
    --include "EXAONE-3.5-7.8B-Instruct-BF16*.gguf" \
    --local-dir .
```

<br>

## Run Locally

For end users, we introduce two ways to run EXAONE 3.5 models locally. 

> [!Note]
> We highly recommend to use repetition penalty not exceeding 1.0 for better generation quality.

### llama.cpp

You can run EXAONE models with llama.cpp as follows:

1. Install llama.cpp. Please refer to the [llama.cpp repository](https://github.com/ggerganov/llama.cpp) for more details.

2. Download EXAONE 3.5 model in GGUF format.

```bash
huggingface-cli download LGAI-EXAONE/EXAONE-3.5-7.8B-Instruct-GGUF \
    --include "EXAONE-3.5-7.8B-Instruct-BF16*.gguf" \
    --local-dir .
```

3. Run the model with llama.cpp in conversational mode.

```bash
llama-cli -cnv -m ./EXAONE-3.5-7.8B-Instruct-BF16.gguf \
    -p "You are EXAONE model from LG AI Research, a helpful assistant."
```

- In case of using EXAONE 3.5 32B model with BF16 precision, you may need to download all split files and merge them before running the model.

    ```bash
    # Download all split files
    huggingface-cli download LGAI-EXAONE/EXAONE-3.5-32B-Instruct-GGUF \
        --include "EXAONE-3.5-32B-Instruct-BF16*.gguf" \
        --local-dir .

    # Merge all split files
    llama-gguf-split --merge \
        ./EXAO

<!-- truncated for clean-room docs-read -->
