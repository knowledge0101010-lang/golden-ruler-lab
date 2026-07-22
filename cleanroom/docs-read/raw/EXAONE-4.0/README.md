# EXAONE-4.0
<br>
<br>
<p align="center">
<img src="assets/EXAONE_Symbol+BI_3d.png", width="400", style="margin: 40 auto;">
<br>
<!-- <p align="center"> 🤗 <a href="https://huggingface.co/collections/LGAI-EXAONE/exaone-40-686b2e0069800c835ed48375">Hugging Face</a> &nbsp | &nbsp 📝 <a href="https://www.lgresearch.ai/blog/view?seq=576"> Blog</a> &nbsp | &nbsp 📑 <a href="https://arxiv.org/abs/2507.11407"> Technical Report </a>-->
<br>
<br>

<div align="center">
  <a href="https://huggingface.co/collections/LGAI-EXAONE/exaone-40-686b2e0069800c835ed48375" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/🤗-Huggingface-FC926C?style=for-the-badge" alt="Huggingface">
  </a>
  <a href="https://www.lgresearch.ai/blog/view?seq=576" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/📝-Blog-E343BD?style=for-the-badge" alt="Blog">
  </a>
  <a href="https://arxiv.org/abs/2507.11407" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/📑-Technical_Report-684CF4?style=for-the-badge" alt="Technical Report">
  </a>
  <a href="https://friendli.ai/suite/~/serverless-endpoints/LGAI-EXAONE/EXAONE-4.0-32B/overview" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/✈️_commercial_purposes-Try_on_FriendliAI-2649BC?style=for-the-badge" alt="FriendliAI">
  </a>
</div>

<br>

| 🎉 License Updated! We are pleased to announce our more flexible licensing terms 🤗 [What's Different?](#license) |
|:---:|

<br>

## Introduction

We introduce **EXAONE 4.0**, which integrates a **Non-reasoning mode** and **Reasoning mode** to achieve both the excellent usability of [EXAONE 3.5](https://github.com/LG-AI-EXAONE/EXAONE-3.5) and the advanced reasoning abilities of [EXAONE Deep](https://github.com/LG-AI-EXAONE/EXAONE-Deep). To pave the way for the agentic AI era, EXAONE 4.0 incorporates essential features such as agentic tool use, and its multilingual capabilities are extended
to support Spanish in addition to English and Korean. 

The EXAONE 4.0 model series consists of two sizes: a mid-size **32B** model optimized for high performance, and a small-size **1.2B** model designed for on-device applications.

In the EXAONE 4.0 architecture, we apply new architectural changes compared to previous EXAONE models as below:

1. **Hybrid Attention**: For the 32B model, we adopt hybrid attention scheme, which combines *Local attention (sliding window attention)* with *Global attention (full attention)* in a 3:1 ratio. We do not use RoPE (Rotary Positional Embedding) for global attention for better global context understanding.
2. **QK-Reorder-Norm**: We reorder the LayerNorm position from the traditional Pre-LN scheme by applying LayerNorm directly to the attention and MLP outputs, and we add RMS normalization right after the Q and K projection. It helps yield better performance on downstream tasks despite consuming more computation.

For more details, please refer to our [technical report](https://arxiv.org/abs/2507.11407).

<br>

## News

- 2025.07.26 : 🌟 EXAONE 4.0 is officially supported by HuggingFace transformers! Please check out the [v4.54.0 release](https://github.com/huggingface/transformers/releases/tag/v4.54.0).
- 2025.07.18 : EXAONE 4.0 is officially supported by [llama.cpp](https://github.com/ggml-org/llama.cpp)! Please check out the released version [here](https://github.com/ggml-org/llama.cpp/releases/tag/b5932).
- 2025.07.15 : We release EXAONE 4.0, a hybrid reasoning model with enhanced usability including 32B and 1.2B. Please check out [these models](https://huggingface.co/collections/LGAI-EXAONE/exaone-40-686b2e0069800c835ed48375)!

<br>

## Performance

The following tables show the evaluation results of each model, with reasoning and non-reasoning mode. The evaluation details can be found in the [technical report](https://arxiv.org/abs/2507.11407).

- ✅ denotes the model has a hybrid reasoning capability, evaluated by selecting reasoning / non-reasoning on the purpose.
- To assess Korean **practical** and **professional** knowledge, we adopt both the [KMMLU-Redux](https://huggingface.co/datasets/LGAI-EXAONE/KMMLU-Redux) and [KMMLU-Pro](https://huggingface.co/datasets/LGAI-EXAONE/KMMLU-Pro) benchmarks. Both datasets are publicly released!


### 32B Reasoning Mode

<table>
    <tr>
        <th> </th>
        <th>EXAONE 4.0 32B </th>
        <th>Phi 4 reasoning-plus</th>
        <th>Magistral Small-2506</th>
        <th>Qwen 3 32B </th>
        <th>Qwen 3 235B </th>
        <th>DeepSeek R1-0528</th>
    </tr>
    <tr>
        <td align="center">Model Size</td>
        <td align="center">32.0B</td>
        <td align="center">14.7B</td>
        <td align="center">23.6B</td>
        <td align="center">32.8B</td>
        <td align="center">235B</td>
        <td align="center">671B</td>
    </tr>
    <tr>
        <td align="center">Hybrid Reasoning</td>
        <td align="center">✅</td>
        <td align="center"> </td>
        <td align="center"> </td>
        <td align="center">✅</td>
        <td align="center">✅</td>
        <td align="center"> </td>
    </tr>
    <tr>
        <td align="center" colspan='7'><i>World Knowledge</i></td>
    </tr>
    <tr>
        <td >MMLU-Redux</td>
        <td align="center">92.3</td>
        <td align="center">90.8</td>
        <td align="center">86.8</td>
        <td align="center">90.9</td>
        <td align="center">92.7</td>
        <td align="center">93.4</td>
    </tr>
    <tr>
        <td >MMLU-Pro</td>
        <td align="center">81.8</td>
        <td align="center">76.0</td>
        <td align="center">73.4</td>
        <td align="center">80.0</td>
        <td align="center">83.0</td>
        <td align="center">85.0</td>
    </tr>
    <tr>
        <td >GPQA-Diamond</td>
        <td align="center">75.4</td>
        <td align="center">68.9</td>
        <td align="center">68.2</td>
        <td align="center">68.4</td>
        <td align="center">71.1</td>
        <td align="center">81.0</td>
    </tr>
    <tr>
        <td align="center" colspan='7'><i>Math/Coding</i></td>
    </tr>
    <tr>
        <td >AIME 2025</td>
        <td align="center">85.3</td>
        <td align="center">78.0</td>
        <td align="center">62.8</td>
        <td align="center">72.9</td>
        <td align="center">81.5</td>
        <td align="center">87.5</td>
    </tr>
    <tr>
        <td >HMMT Feb 2025</td>
        <td align="center">72.9</td>
        <td align="center">53.6</td>
        <td align="center">43.5</td>
        <td align="center">50.4</td>
        <td align="center">62.5</td>
        <td align="center">79.4</td>
    </tr>
    <tr>
        <td >LiveCodeBench v5</td>
        <td align="center">72.6</td>
        <td align="center">51.7</td>
        <td align="center">55.8</td>
        <td align="center">65.7</td>
        <td align="center">70.7</td>
        <td align="center">75.2</td>
    </tr>
    <tr>
        <td >LiveCodeBench v6</td>
        <td align="center">66.7</td>
        <td align="center">47.1</td>
        <td align="center">47.4</td>
        <td align="center">60.1</td>
        <td align="center">58.9</td>
        <td align="center">70.3</td>
    </tr>
    <tr>
        <td align="center" colspan='7'><i>Instruction Following</i></td>
    </tr>
    <tr>
        <td >IFEval</td>
        <td align="center">83.7</td>
        <td align="center">84.9</td>
        <td align="center">37.9</td>
        <td align="center">85.0</td>
        <td align="center">83.4</td>
        <td align="center">80.8</td>
    </tr>
    <tr>
        <td >Multi-IF (EN)</td>
        <td align="center">73.5</td>
        <td align="center">56.1</td>
        <td align="center">27.4</td>
        <td align="center">73.4</td>
        <td align="center">73.4</td>
        <td align="center">72.0</td>
    </tr>
    <tr>
        <td align="center" colspan='7'><i>Agentic Tool Use</i></td>
    </tr>
    <tr>
        <td >BFCL-v3</td>
        <td align="center">63.9</td>
        <td align="center">N/A</td>
        <td align="center">40.4</td>
        <td align="center">70.3</td>
        <td align="center">70.8</td>
        <td align="center">64.7</td>
    </tr>
    <tr>
        <td >Tau-Bench (Airline)</td>
        <td align="center">51.5</td>
        <td align="center">N/A</td>
        <td align="center">38.5</td>
        <td align="center">34.5</td>
        <td align="center">37.5</td>
        <td align="center">53.5</td>
    </tr>
    <tr>
        <td >Tau-Bench (Retail)</td>
        <td align="center">62.8</td>
        <td align="center">N/A</td>
        <td align="center">10.2</td>
        <td align="center">55.2</td>
        <td align="center">58.3</td>
        <td align="center">63.9</td>
    </tr>
    <tr>
        <td align="center" colspan='7'><i>Multilinguality</i></td>
    </tr>
    <tr>
        <td >KMMLU-Pro</td>
        <td align="center">67.7</td>
        <td align="center">55.8</td>
        <td align="center">51.5</td>
        <td align="center">61.4</td>
        <td align="center">68.1</td>
        <td align="center">71.7</td>
    </tr>
    <tr>
        <td >KMMLU-Redux</td>
        <td align="center">72.7</td>
        <td align="center">62.7</td>
        <td align="center">54.6</td>
        <td align="center">67.5</td>
        <td align="center">74.5</td>
        <td align="center">77.0</td>
    </tr>
    <tr>
        <td >KSM</td>
        <td align="center">87.6</td>
        <td align="center">79.8</td>
        <td align="center">71.9</td>
        <td align="center">82.8</td>
        <td align="center">86.2</td>
        <td align="center">86.7</td>
    </tr>
    <tr>
        <td >MMMLU (ES)</td>
        <td align="center">85.6</td>
        <td align="center">84.3</td>
        <td align="center">68.9</td>
        <td align="center">82.8</td>
        <td align="center">86.7</td>
        <td align="center">88.2</td>
    </tr>
    <tr>
        <td >MATH500 (ES)</td>
        <td align="center">95.8</td>
        <td align="center">94.2</td>
        <td align="center">83.5</td>
        <td align="center">94.3</td>
        <td align="center">95.1</td>
        <td align="center">96.0</td>
    </tr>
</table>

### 32B Non-Reasoning Mode

<table>
    <tr>
        <th> </th>
        <th>EXAONE 4.0 32B </th>
        <th>Phi 4</th>
        <th>Mistral-Small-2506</th>
        <th>Gemma3 27B</th>
        <th>Qwen3 32B </th>
        <th>Qwen3 235B </th>
        <th>Llama-4-Maverick</th>
        <th>DeepSeek V3-0324</th>
    </tr>
    <tr>
        <td align="center">Model Size</td>
        <td align="center">32.0B</td>
        <td align="center">14.7B</td>
        <td align="center">24.0B</td>
        <td align="center">27.4B</td>
        <td align="center">32.8B</td>
        <td align="center">235B</td>
        <td align="center">402B</td>
        <td align="center">671B</td>
    </tr>
    <tr>
        <td align="center">Hybrid Reasoning</td>
        <td align="center">✅</td>
        <td align="center"> </td>
        <td align="center"> </td>
        <td align="center"> </td>
        <td align="center">✅</td>
        <td align="center">✅</td>
        <td align="center"> </td>
        <td align="center"> </td>
    </tr>
    <tr>
        <td align="center" colspan='9'><i>World Knowledge</i></td>
    </tr>
    <tr>
        <td >MMLU-Redux</td>
        <td align="center">89.8</td>
        <td align="center">88.3</td>
        <td align="center">85.9</td>
        <td align="center">85.0</td>
        <td align="center">85.7</td>
        <td align="center">89.2</td>
        <td align="center">92.3</td>
        <td align="center">92.3</td>
    </tr>
    <tr>
        <td >MMLU-Pro</td>
        <td align="center">77.6</td>
        <td align="center">70.4</td>
        <td align="center">69.1</td>
        <td align="center">67.5</td>
        <td align="center">74.4</td>
        <td align="center">77.4</td>
        <td align="center">80.5</td>
        <td align="center">81.2</td>
    </tr>
    <tr>
        <td >GPQA-Diamond</td>
   

<!-- truncated for clean-room docs-read -->
