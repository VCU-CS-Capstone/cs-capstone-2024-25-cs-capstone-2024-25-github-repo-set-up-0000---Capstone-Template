# GPT-4
## Getting started
```
pip install openai
```
- WIP. Requires subscription

# Llama 2
## Overview

![alt text](fine-tuning.png)
- Pre-trained (for NLP tasks) and fine-tuned (for chat tasks)
- Increased context length

### Local Download:
After downloading from Meta, type in the terminal:
```
pip install wget
```
Open Powershell and type to install a Linux subsystem in Windows:
```
wsl --install
wsl --set-default-version 2
```

### HuggingFace download:
In the terminal type:
```
huggingface-cli login
```
And right-click to paste log-in key from August

### Install problem (solved)
ML models commonly use Torch packages. To get llama running I had to uninstall all Torch related packages and re-install from a specific link
<br>1.)
```
py -m pip uninstall torch torchvision torchaudio
```
2.)
```
py -m pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121
```