# KMMLU-Pro Evaluation Script

**Language**: [English](README.md) | [한국어](README_ko.md)

📄 [Paper](https://arxiv.org/abs/2507.08924) | 📚 [Dataset](https://huggingface.co/datasets/LGAI-EXAONE/KMMLU-Pro)

## Overview

**KMMLU-Pro** is a challenging benchmark comprising 2,822 problems from the 2024 Korean National Professional Licensure (KNPL) official exams, representing highly specialized professions in Korea.
This repository provides evaluation scripts to generate model responses using the OpenAI-compatible interface and calculate professional license pass/fail results.

## Setup

### Prerequisites

1. **Dataset Access**: Request access to the [KMMLU-Pro dataset](https://huggingface.co/datasets/LGAI-EXAONE/KMMLU-Pro) on Hugging Face.

2. **OpenAI API Key**: Set your OpenAI API key as an environment variable:
   ```bash
   export OPENAI_API_KEY="your-api-key-here"
   ```

3. **Installation**:
   ```bash
   git clone https://github.com/LG-AI-EXAONE/KMMLU-Pro.git
   cd KMMLU-Pro
   pip install -r requirements.txt
   ```

## Usage

### 1. Generate Model Responses

**Non-Reasoning Model Usage**
```bash
python generate_model_responses.py --model YOUR_MODEL_NAME
```

**Reasoning Model Usage**
```bash
python generate_model_responses.py --model YOUR_MODEL_NAME --temperature 0.6 --top_p 0.95 --enable_reasoning
```

**Additional Options**
- `--model`: Model name (required)
- `--output_dir`: Output directory (default: `./results`)
- `--prompt_language`: Prompt language, 'ko' or 'en' (default: `ko`)
- `--temperature`: Sampling temperature (default: `0.0`)
- `--top_p`: Top-p sampling (default: `1.0`)
- `--presence_penalty`: Presence penalty (default: `0.0`)
- `--max_tokens`: Maximum tokens per response (default: `32768`)
- `--max_requests`: Maximum concurrent requests (default: `200`)
- `--enable_reasoning`: Enable reasoning mode (flag)

### 2. Calculate Scores and License Results

```bash
python print_score.py --model_responses "results/{YOUR_MODEL_NAME}_results.jsonl"
```

## Output

The evaluation provides:
- **Overall Accuracy**: Weighted average across all questions
- **Per-License Results**: Pass/fail status for each of the 14 professional licenses
- **Subject-Level Scores**: Detailed breakdown by license and subject area

### Example Output:
```
Accuracy : 78.09%
    법무사 54.55% Fail
    변호사 49.33% Fail
    공인노무사 71.55% Pass
    ...
Passed Licenses : 10
```

### Supported Licenses

The benchmark evaluates 14 Korean professional licenses:
- 법무사 (Judicial Scrivener)
- 변호사 (Lawyer) 
- 공인노무사 (Certified Public Labor Attorney)
- 변리사 (Certified Patent Attorney)
- 공인회계사 (Certified Public Accountant)
- 세무사 (Certified Tax Accountant)
- 관세사 (Certified Customs Broker)
- 손해사정사 (Certified Damage Adjuster)
- 감정평가사 (Certified Appraiser)
- 한의사 (Doctor of Korean Medicine)
- 치과의사 (Dentist)
- 약사 (Pharmacist)
- 한약사 (Herb Pharmacist)
- 의사 (Physician)
