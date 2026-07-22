<br>
<br>
<p align="center">
<img src="assets/EXAONE_Symbol+BI_3d.png" width="400">
<br>
<br>
<br>

<div align="center">
  <a href="https://huggingface.co/collections/LGAI-EXAONE/exaone-45" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/🤗-HuggingFace-FC926C?style=for-the-badge" alt="HuggingFace">
  </a>
  <a href="https://www.lgresearch.ai/blog/view?seq=641" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/📝-Blog-E343BD?style=for-the-badge" alt="Blog">
  </a>
  <a href="http://arxiv.org/abs/2604.08644" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/📑-Technical_Report-684CF4?style=for-the-badge" alt="Technical Report">
  </a>
  <a href="https://github.com/LG-AI-EXAONE/EXAONE-4.5" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/🖥️-GitHub-2B3137?style=for-the-badge" alt="GitHub">
  </a>
  <!-- <a href="#" style="text-decoration: none;">
    <img src="https://img.shields.io/badge/✈️_API-Try_on_FriendliAI-2649BC?style=for-the-badge" alt="FriendliAI">
  </a> -->
</div>

<br>

<br>


## EXAONE 4.5

We introduce EXAONE 4.5, the first open-weight vision language model developed by LG AI Research.
Integrating a dedicated visual encoder into the existing EXAONE 4.0 framework, we expand the model's capability toward multimodality.
EXAONE 4.5 features 33 billion parameters in total, including 1.2 billion parameters from the vision encoder. 
EXAONE 4.5 achieves competitive performance in general benchmark while outperforming SOTA models of similar size in document understanding and Korean contextual reasoning, inheriting powerful language capabilities from our previous language models.

For more details, please refer to the [technical report](http://arxiv.org/abs/2604.08644), [blog](https://www.lgresearch.ai/blog/view?seq=641) and [GitHub](https://github.com/LG-AI-EXAONE/EXAONE-4.5).




## News

- 2026.04.09 : 🚩 We release EXAONE 4.5, the first open-weight vision language model with 33B params. Please check out [the model](https://huggingface.co/collections/LGAI-EXAONE/exaone-45)
- 2025.12.31 : We release 🇰🇷 K-EXAONE, a 236B MoE model with 23B active params, efficiently scaling model capacity with significant enhancement. Please check out [the model](https://huggingface.co/collections/LGAI-EXAONE/k-exaone)!
- 2025.07.15 : We released EXAONE 4.0, a hybrid reasoning model with enhanced usability including 32B and 1.2B. Please check out [these models](https://huggingface.co/collections/LGAI-EXAONE/exaone-40)!
- 2025.03.18: We released the EXAONE Deep, reasoning enhanced language models, including 2.4B, 7.8B, and 32B. Check out [these models](https://huggingface.co/collections/LGAI-EXAONE/exaone-deep)!
- 2024.12.09: We released the EXAONE 3.5 language model series including 2.4B, 7.8B, and 32B instruction-tuned models. Check out [these models](https://huggingface.co/collections/LGAI-EXAONE/exaone-35)!
- 2024.08.07: We released the EXAONE 3.0 7.8B instruction-tuned model. Check out the [model](https://huggingface.co/collections/LGAI-EXAONE/exaone-30)!

## Performance



### Vision-Language Tasks

<table>
	<tr>
		<th style="background: rgba(128,128,128,0.1); text-align: center;"> </th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">EXAONE 4.5 33B (Reasoning)</th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">GPT-5 mini (Reasoning: high)</th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">Qwen3-VL 32B Thinking</th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">Qwen3-VL 235B Thinking</th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">Qwen3.5 27B (Reasoning)</th>
	</tr>
	<tr>
		<td align="center">Architecture</td>
		<td align="center">Dense</td>
		<td align="center">-</td>
		<td align="center">Dense</td>
		<td align="center">MoE</td>
		<td align="center">Dense</td>
	</tr>
	<tr>
		<td align="center">Total Params</td>
		<td align="center">33B</td>
		<td align="center">-</td>
		<td align="center">33B</td>
		<td align="center">236B</td>
		<td align="center">27B</td>
	</tr>
	<tr>
		<td align="center">Active Params</td>
		<td align="center">33B</td>
		<td align="center">-</td>
		<td align="center">33B</td>
		<td align="center">22B</td>
		<td align="center">27B</td>
	</tr>
	<tr>
		<td align="center" colspan='7' style="background: linear-gradient(90deg, rgba(252,146,108,0.3) 0%, rgba(227,67,189,0.3) 50%, rgba(104,76,244,0.3) 100%); font-weight: bold; height:32px; padding-top:2px; padding-bottom:2px;"><i>STEM / Puzzle</i></td>
	</tr>
	<tr>
		<td align="center">MMMU</td>
		<td align="center">78.7</td>
		<td align="center">79.0</td>
		<td align="center">78.1</td>
		<td align="center">80.6</td>
		<td align="center">82.3</td>
	</tr>
	<tr>
		<td align="center">MMMU-Pro</td>
		<td align="center">68.6</td>
		<td align="center">67.3</td>
		<td align="center">68.1</td>
		<td align="center">69.3</td>
		<td align="center">75.0</td>
	</tr>
	<tr>
		<td align="center">MedXpertQA-MM</td>
		<td align="center">42.1</td>
		<td align="center">34.4</td>
		<td align="center">41.6</td>
		<td align="center">47.6</td>
		<td align="center">62.4</td>
	</tr>
	<tr>
		<td align="center">MathVision</td>
		<td align="center">75.2</td>
		<td align="center">71.9</td>
		<td align="center">70.2</td>
		<td align="center">74.6</td>
		<td align="center">86.0</td>
	</tr>
	<tr>
		<td align="center">MathVista (mini)</td>
		<td align="center">85.0</td>
		<td align="center">79.1</td>
		<td align="center">85.9</td>
		<td align="center">85.8</td>
		<td align="center">87.8</td>
	</tr>
	<tr>
		<td align="center">WeMath</td>
		<td align="center">79.1</td>
		<td align="center">70.3</td>
		<td align="center">71.6</td>
		<td align="center">74.8</td>
		<td align="center">84.0</td>
	</tr>
	<tr>
		<td align="center">LogicVista</td>
		<td align="center">73.8</td>
		<td align="center">70.3</td>
		<td align="center">70.9</td>
		<td align="center">72.2</td>
		<td align="center">77.0</td>
	</tr>
	<tr>
		<td align="center">BabyVision</td>
		<td align="center">18.8</td>
		<td align="center">20.9</td>
		<td align="center">17.4</td>
		<td align="center">22.2</td>
		<td align="center">44.6</td>
	</tr>
	<tr>
		<td align="center" colspan='7' style="background: linear-gradient(90deg, rgba(252,146,108,0.3) 0%, rgba(227,67,189,0.3) 50%, rgba(104,76,244,0.3) 100%); font-weight: bold; height:32px; padding-top:2px; padding-bottom:2px;"><i>Document Understanding</i></td>
	</tr>
	<tr>
		<td align="center">AI2D</td>
		<td align="center">89.0</td>
		<td align="center">88.2</td>
		<td align="center">88.9</td>
		<td align="center">89.2</td>
		<td align="center">92.9</td>
	</tr>
	<tr>
		<td align="center">ChartQAPro</td>
		<td align="center">62.2</td>
		<td align="center">60.9</td>
		<td align="center">61.4</td>
		<td align="center">61.2</td>
		<td align="center">66.8</td>
	</tr>
	<tr>
		<td align="center">CharXiv (RQ)</td>
		<td align="center">71.7</td>
		<td align="center">68.6</td>
		<td align="center">65.2</td>
		<td align="center">66.1</td>
		<td align="center">79.5</td>
	</tr>
	<tr>
		<td align="center">OCRBench v2</td>
		<td align="center">63.2</td>
		<td align="center">55.8</td>
		<td align="center">68.4</td>
		<td align="center">66.8</td>
		<td align="center">67.3</td>
	</tr>
	<tr>
		<td align="center">OmniDocBench v1.5</td>
		<td align="center">81.2</td>
		<td align="center">77.0</td>
		<td align="center">83.1</td>
		<td align="center">84.5</td>
		<td align="center">88.9</td>
	</tr>
	<tr>
		<td align="center" colspan='7' style="background: linear-gradient(90deg, rgba(252,146,108,0.3) 0%, rgba(227,67,189,0.3) 50%, rgba(104,76,244,0.3) 100%); font-weight: bold; height:32px; padding-top:2px; padding-bottom:2px;"><i>General</i></td>
	</tr>
	<tr>
		<td align="center">MMStar</td>
		<td align="center">74.9</td>
		<td align="center">74.1</td>
		<td align="center">79.4</td>
		<td align="center">78.7</td>
		<td align="center">81.0</td>
	</tr>
	<tr>
		<td align="center">BLINK</td>
		<td align="center">68.8</td>
		<td align="center">67.7</td>
		<td align="center">68.5</td>
		<td align="center">67.1</td>
		<td align="center">71.6</td>
	</tr>
	<tr>
		<td align="center">HallusionBench</td>
		<td align="center">63.7</td>
		<td align="center">63.2</td>
		<td align="center">67.4</td>
		<td align="center">66.7</td>
		<td align="center">70.0</td>
	</tr>
	<tr>
		<td align="center" colspan='7' style="background: linear-gradient(90deg, rgba(252,146,108,0.3) 0%, rgba(227,67,189,0.3) 50%, rgba(104,76,244,0.3) 100%); font-weight: bold; height:32px; padding-top:2px; padding-bottom:2px;"><i>Korean</i></td>
	</tr>
	<tr>
		<td align="center">KMMMU</td>
		<td align="center">42.7</td>
		<td align="center">42.6</td>
		<td align="center">37.8</td>
		<td align="center">42.1</td>
		<td align="center">51.7</td>
	</tr>
	<tr>
		<td align="center">K-Viscuit</td>
		<td align="center">80.1</td>
		<td align="center">78.5</td>
		<td align="center">78.5</td>
		<td align="center">83.9</td>
		<td align="center">84.0</td>
	</tr>
	<tr>
		<td align="center">KRETA</td>
		<td align="center">91.9</td>
		<td align="center">94.8</td>
		<td align="center">90.3</td>
		<td align="center">92.8</td>
		<td align="center">96.5</td>
	</tr>
</table>


### Language-only Tasks

<table>
	<tr>
		<th style="background: rgba(128,128,128,0.1); text-align: center;"> </th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">EXAONE 4.5 33B (Reasoning)</th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">GPT-5 mini (Reasoning: high)</th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">K-EXAONE 236B (Reasoning)</th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">Qwen3-VL 235B Thinking</th>
		<th style="background: rgba(128,128,128,0.1); text-align: center;">Qwen3.5 27B (Reasoning)</th>
	</tr>
	<tr>
		<td align="center">Architecture</td>
		<td align="center">Dense</td>
		<td align="center">-</td>
		<td align="center">MoE</td>
		<td align="center">MoE</td>
		<td align="center">Dense</td>
	</tr>
	<tr>
		<td align="center">Total Params</td>
		<td align="center">33B</td>
		<td align="center">-</td>
		<td align="center">236B</td>
		<td align="center">236B</td>
		<td align="center">27B</td>
	</tr>
	<tr>
		<td align="center">Active Params</td>
		<td align="center">33B</td>
		<td align="center">-</td>
		<td align="center">23B</td>
		<td align="center">22B</td>
		<td align="center">27B</td>
	</tr>
	<tr>
		<td align="center" colspan='7' style="background: linear-gradient(90deg, rgba(252,146,108,0.3) 0%, rgba(227,67,189,0.3) 50%, rgba(104,76,244,0.3) 100%); font-weight: bold; height:32px; padding-top:2px; padding-bottom:2px;"><i>Reasoning</i></td>
	</tr>
	<tr>
		<td align="center">AIME 2025</td>
		<td align="center">92.9</td>
		<td align="center">91.1</td>
		<td align="center">92.8</td>
		<td align="center">89.7</td>
		<td align="center">93.5</td>
	</tr>
	<tr>
		<td align="center">AIME 2026</td>
		<td align="center">92.6</td>
		<td align="center">92.4</td>
		<td align="center">92.2</td>
		<td align="center">89.4</td>
		<td align="center">90.8</td>
	</tr>
	<tr>
		<td align="center">GPQA-Diamond</td>
		<td align="center">80.5</td>
		<td align="center">82.3</td>
		<td align="center">79.1</td>
		<td align="center">77.1</td>
		<td align="center">85.5</td>
	</tr>
	<tr>
		<td align="center">LiveCodeBench v6</td>
		<td align="center">81.4</td>
		<td align="center">78.1</td>
		<td align="center">80.7</td>
		<td align="center">70.1</td>
		<td align="center">80.7</td>
	</tr>
	<tr>
		<td align="center">MMLU-Pro</td>
		<td align="center">83.3</td>
		<td align="center">83.3</td>
		<td align="center">83.8</td>
		<td align="center">83.8</td>
		<td align="center">86.1</td>
	</tr>
	<tr>
		<td align="center" colspan='7' style="background: linear-gradient(90deg, rgba(252,146,108,0.3) 0%, rgba(227,67,189,0.3) 50%, rgba(104,76,244,0.3) 100%); font-weight: bold; height:32px; padd

<!-- truncated for clean-room docs-read -->
