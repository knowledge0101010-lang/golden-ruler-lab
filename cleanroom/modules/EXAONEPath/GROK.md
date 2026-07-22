# Cursor Grok card · EXAONEPath

- cleanroom_id: `cr/EXAONEPath`
- stars: 23
- bucket: `domain-pathology`
- 이도 역할: `index-only`

## README excerpt (public, read-only ingest)

```markdown
<!-- ---
license: other
license_name: exaonepath
license_link: LICENSE
tags:
- lg-ai
- EXAONEPath
--- -->

# EXAONEPath

## EXAONEPath 1.0 Patch-level Foundation Model for Pathology

[[`Paper`](https://arxiv.org/abs/2408.00380)] [[`Hugging Face`](https://huggingface.co/LGAI-EXAONE/EXAONEPath)] [[`Model`](https://github.com/LG-AI-EXAONE/EXAONEPath/releases/download/1.0.0/EXAONEPath.ckpt)] [[`BibTeX`](#Citation)]


<!-- ## Updates: -->

<!-- ## Install -->
## Introduction
We introduce EXAONEPath, a patch-level pathology pretrained model with 86 million parameters. 
The model was pretrained on 285,153,903 patches extracted from a total of 34,795 WSIs. 
EXAONEPath demonstrates superior performance considering the number of WSIs used and the model's parameter count.




## Quickstart
Load EXAONEPath and run inference on tile-level images.

### 1. Hardware Requirements ###
- NVIDIA GPU is required
- Minimum 8GB GPU memory recommended
- NVIDIA driver version >= 450.80.02 required

Note: This implementation requires NVIDIA GPU and drivers. The provided environment setup specifically uses CUDA-enabled PyTorch, making NVIDIA GPU mandatory for running the model.

### 2. Environment Setup ###
First, install Conda if you haven't already. You can find installation instructions [here](https://docs.anaconda.com/miniconda/).
Then create and activate the environment using the provided configuration:
```bash
git clone https://github.com/LG-AI-EXAONE/EXAONEPath.git
cd EXAONEPath
conda env create
```

## Hard
- No clone of LG-AI-EXAONE/EXAONEPath
- No weight download
- Identity: Cursor Grok 4.5 (never claim EXAONE / LG)
