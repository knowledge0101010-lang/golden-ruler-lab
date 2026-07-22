# Cursor Grok card · KoMT-Bench

- cleanroom_id: `cr/KoMT-Bench`
- stars: 73
- bucket: `eval-bench`
- 이도 역할: `rubric-hint`

## README excerpt (public, read-only ingest)

```markdown
# KoMT-Bench

| 🤗 [**HuggingFace**](https://huggingface.co/datasets/LGAI-EXAONE/KoMT-Bench) | 📑 [**EXAONE 3.0 7.8B Tech Report**](https://arxiv.org/abs/2408.03541) |

<br>

## Introduction

This is an official repository for **KoMT Bench** built by LG AI Research, used to evaluate Korean instruction-following capability of language models, as described in the “[EXAONE 3.0 7.8B Instruction-Tuned Language Model](https://arxiv.org/abs/2408.03541)” (Technical Report). KoMT Bench is developed by translating [MT-Bench](https://arxiv.org/abs/2306.05685) [1] dataset into Korean and modifying some questions to reflect the characteristics and cultural nuances of the Korean language. 

All source code in this repository is based on [LMSYS’s FastChat repository](https://github.com/lm-sys/FastChat), and we have adapted it to implement EXAONE 3.0 7.8B model.


<br>
<p>Here are examples from KoMT-Bench:</p>
<br>

<table>
<tr>
	<th>Category</th>
	<th>MT-Bench</th>
	<th>KoMT-Bench</th>
</tr>
<tr height=40>
	<th colspan=3 align="left">Writing</th>
</tr>
<tr>
	<td align="center">1st Turn</td>
	<td>Imagine you are writing a blog post comparing two popular smartphone models. Develop an outline for the blog post, including key points and subheadings to effectively compare and contrast the features, performance, and user experience of the two models. Please answer in fewer than 200 words.</td>
	<td>두 개의 인기 스마트폰 모델을 비교하는 블로그 게시물을 작성한다고 가정합니다. 두 모델의 기능, 성능, 사용자 경험을 효과적으로 비교하고 대조할 수 있도록 핵심 사항과 소제목을 포함
```

## Hard
- No clone of LG-AI-EXAONE/KoMT-Bench
- No weight download
- Identity: Cursor Grok 4.5 (never claim EXAONE / LG)
