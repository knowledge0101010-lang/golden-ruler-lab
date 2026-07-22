# Cursor Grok card · EXAONE-Path-2.5

- cleanroom_id: `cr/EXAONE-Path-2.5`
- stars: 5
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
- EXAONE-Path-2.5
- pathology
--- -->


# EXAONE Path 2.5
[[`Github`](https://github.com/LG-AI-EXAONE/EXAONE-Path-2.5)] [[`Hugging Face`](https://huggingface.co/LGAI-EXAONE/EXAONE-Path-2.5)] [[`Paper`](https://arxiv.org/abs/2512.14019)] [[`Cite`](#citation)]

## Introduction
EXAONE Path 2.5 is a biologically informed multimodal framework that enriches histopathology representations by aligning whole-slide images with *genomic, epigenetic, and transcriptomic data*. By enabling all-pairwise cross-modal alignment across multiple layers of tumor biology, the model captures coherent genotype-to-phenotype relationships within a unified embedding space. This domain-informed design improves resource efficiency, enabling the model to achieve competitive performance across diverse tasks while using substantially fewer training samples and parameters than existing approaches.

<div align="center">
    <img src="https://raw.githubusercontent.com/LG-AI-EXAONE/EXAONE-Path-2.5/main/figs/exaonepath_2.5.png" width="70%"><br>
  <em>Figure 1. Overall scheme of EXAONE Path 2.5.</em>
</div>


## Quickstart
Load EXAONE Path 2.5 and extract features.

### 1. Hardware Requirements ###
- NVIDIA GPU with 12GB+ VRAM
- NVIDIA driver version >= 525.60.13 required

Note: This implementation requires NVIDIA GPU and drivers. The provided environment setup specifically uses CUDA-enabled PyTorch, making NVIDIA GPU mandatory f
```

## Hard
- No clone of LG-AI-EXAONE/EXAONE-Path-2.5
- No weight download
- Identity: Cursor Grok 4.5 (never claim EXAONE / LG)
