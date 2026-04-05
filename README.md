# <p align="center">🌃 VOID's Tokyo Night Dotfiles</p>

<p align="center">
  <img src="https://img.shields.io/badge/OS-Arch_Linux-7aa2f7?style=for-the-badge&logo=arch-linux&logoColor=white" />
  <img src="https://img.shields.io/badge/WM-Hyprland-bb9af7?style=for-the-badge&logo=hyprland&logoColor=white" />
  <img src="https://img.shields.io/badge/Shell-Zsh-F1502F?style=for-the-badge&logo=zsh&logoColor=white" />
  <img src="https://img.shields.io/badge/Terminal-Kitty-7dcfff?style=for-the-badge&logo=kitty&logoColor=white" />
  <img src="https://img.shields.io/badge/Backend-Rust-f7768e?style=for-the-badge&logo=rust&logoColor=white" />
</p>

<p align="center">
  <i>"Arquitectura de sistema minimalista, modular y centrada en el rendimiento."</i>
</p>

---

## 🌙 Overview

Este repositorio contiene la **V1.0** de mis configuraciones personales (dotfiles). A diferencia de los setups convencionales, este entorno está gestionado bajo una política de **Whitelist**, asegurando que solo el código y la lógica de configuración crítica sean versionados.

### 🎨 Paleta Tokyo Night
- **Primary Blue:** `#7aa2f7` (Storm)
- **Accent Purple:** `#bb9af7` (Visual)
- **Terminal Cyan:** `#7dcfff` (Zsh Accents)
- **Critical Red:** `#f7768e` (Rust Error Handling)

---

## ⚙️ System Architecture (The Rust Engine)

Como desarrollador Backend, la automatización del sistema se delega en herramientas nativas escritas en **Rust**, integradas con el flujo de trabajo de **Zsh + Oh My Zsh**:

* **`device_notifier`** 📱: Monitor de eventos USB y gestión de protocolos MTP.
* **`sunset`** 🌇: Orquestador de temperatura de color nocturna (`3800K`).
* **`mediaplayer`** 🎵: Puente de control MPRIS para la interfaz de Waybar.
* **`touchpad` / `numlock_status`** ⌨️: Controladores de estado de hardware.

---

## 📂 Repository Structure

La estructura refleja una organización modular compatible con el ecosistema de Arch:

```bash
~/.config/
├──  hypr/             # Lógica de ventanas y binds (Hyprland)
├──  waybar/           # Módulos de barra de estado y estilos CSS
├──  kitty/            # Emulador de terminal acelerado por GPU
├── 󱔗 system_scripts/    # Microservicios de sistema (Rust Projects)
├── 󰀻 wlogout/          # Menú de sesión y assets visuales
└──  rofi/             # Lanzador de aplicaciones y menús dmenu
~/.oh-my-zsh/           # Framework de shell (Instalación externa)
~/.zshrc                # Configuración de alias y plugins de backend
