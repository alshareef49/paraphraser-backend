# 🚀 AI Paraphraser Backend (Rust + Rocket)

This is the backend API for the **AI Paraphraser** application. It accepts a block of text and returns a paraphrased version using an AI model (e.g., OpenAI). Built with **Rust**, **Rocket**, and deployed using **Shuttle**.

---

## 📦 API Overview

### `POST /paraphrase`

Accepts a JSON request with a `text` field and returns a paraphrased version.

#### ✅ Request Example

```json
{
  "text": "I went to the store to buy some milk."
}
```

#### 🔁 Response Example

```json
{
  "paraphrased": "I visited a store in order to purchase some milk"
}
```

---

## 🛠️ Tech Stack

- **Rust**
- **Rocket** – Web framework
- **Serde** – JSON parsing
- **reqwest** – For calling AI APIs like OpenAI
- **Shuttle** – Rust serverless deployment
- **tokio** – Async runtime

---

## 🧪 Running Locally

### 📋 Prerequisites

- Rust (with `cargo`)
- Rocket (`cargo add rocket`)
- Shuttle CLI:
  ```bash
  cargo install cargo-shuttle
  ```

### ▶️ Start Locally

```bash
cargo shuttle run
```

Rocket will start at:

```
http://localhost:8000
```

---

## 🚀 Deploying to Shuttle

```bash
cargo shuttle deploy
```

---

## 🔐 Environment Variables

Make sure your API key (e.g., OpenAI) is configured as a secret:

```bash
cargo shuttle secrets add OPENAI_API_KEY
```

To view secrets:

```bash
cargo shuttle secrets list
```

---

## 📁 Example File Structure

```
src/
├── main.rs          # Rocket setup and route mounting
```

---



## 👤 Author

Shareef Alam