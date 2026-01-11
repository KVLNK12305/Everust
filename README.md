# Everust 🦀

**Everust** is a long-term, systems-oriented project that documents and operationalizes my journey of learning **Rust** by turning concepts into a real, end-to-end system.

Instead of isolated snippets, Everust connects **Rust fundamentals, experiments, benchmarks, and a live web interface**, backed by a Rust API.

> *Learn Rust the way it is used in real systems.*

---

## ✨ Project Philosophy

- Treat learning code as **first-class engineering artifacts**
- Keep **foundations pure**, production code isolated
- Emphasize **correctness, performance, and clarity**
- Evolve deliberately from fundamentals → systems → security

Everust is designed to scale alongside my understanding of Rust.

---

## 🗂 Repository Structure

```text
everust/
├─ foundations/        # Core Rust learning code (concept-focused, framework-free)
├─ backend/            # Rust backend API (Axum)
├─ web/                # Frontend (SvelteKit + Tailwind CSS)
├─ experiments/        # Systems-level and exploratory Rust experiments
├─ benches/            # Performance benchmarks and comparisons
└─ README.md
```

---

## 📁 Directory Intent

### `foundations/`
- Ownership, borrowing, and lifetimes
- Traits, enums, and pattern matching
- Memory model and concurrency basics  
*This directory intentionally avoids frameworks and abstractions.*

### `backend/`
- Production-style Rust backend using **Axum**
- Async APIs, routing, and structured responses
- Logging, tracing, and future middleware integration

### `web/`
- Minimal UI built with **SvelteKit**
- Tailwind CSS for rapid, consistent styling
- Consumes and visualizes Rust backend APIs

### `experiments/`
- Unsafe Rust explorations
- Concurrency models and synchronization
- Memory layout, cache behavior, and performance testing

### `benches/`
- Criterion-based benchmarks
- Comparative performance analysis
- Visualization-ready benchmark outputs

---

## 🛠 Tech Stack

### Backend
- **Rust**
- **Axum** (HTTP API framework)
- **Tokio** (async runtime)
- **Tower** (middleware ecosystem)

### Frontend
- **SvelteKit**
- **Tailwind CSS**

### Tooling
- Cargo
- Git & GitHub
- Linux-based development workflow

---

## 🚀 Current Status

- ✅ Repository structure finalized
- ✅ Foundations module actively growing
- 🚧 Backend API scaffolding in progress
- 🚧 Frontend integration planned
- 🔜 Benchmarks and visualizations

This project is intentionally iterative and evolving.

---

## 📌 Planned Features

- REST APIs exposing Rust learning modules
- Visual representation of benchmark results
- Experiment documentation rendered via the web UI
- Performance comparisons between Rust approaches
- Systems- and security-focused Rust experiments
- Observability: logging, tracing, and metrics

---

## 🎯 Why This Project Exists

Most learning repositories stop at syntax.  
Everust exists to explore deeper questions:

- How does Rust behave under real workloads?
- How do ownership and lifetimes scale in production code?
- What trade-offs exist between safety and performance?
- How can Rust be used to build secure backend systems?

This project serves as both a **learning archive** and a **living system**.

---

## 📄 Resume Description (Suggested)

> **Everust** — Rust Systems Learning Platform  
> Designed and implemented a modular Rust-based project combining foundational Rust concepts, a production-style Axum backend, and a modern SvelteKit frontend. Focused on memory safety, async systems, performance benchmarking, and long-term maintainability.

---

## 📬 Notes

- This repository is structured for **long-term growth**
- Code quality and clarity are prioritized over feature velocity
- Foundations code remains framework-free by design

---

## 🦀 Author

**Venkata Lakshmi Narasimha Kushal Kurapati**  
Rust • Systems • Performance • Security

---

> *Everust is not a demo project.  
> It is a system that grows as my understanding grows.*

