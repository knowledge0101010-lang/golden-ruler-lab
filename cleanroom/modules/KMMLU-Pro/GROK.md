# Cursor Grok card · KMMLU-Pro

- cleanroom_id: `cr/KMMLU-Pro`
- stars: 16
- bucket: `eval-bench`
- 이도 역할: `rubric-hint`

## README excerpt (public, read-only ingest)

```markdown
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
- `--prompt_language`: Prompt language, 'ko' or 'en' (def
```

## Hard
- No clone of LG-AI-EXAONE/KMMLU-Pro
- No weight download
- Identity: Cursor Grok 4.5 (never claim EXAONE / LG)
