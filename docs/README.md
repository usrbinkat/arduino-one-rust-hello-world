# Documentation Guide

Welcome to the **Documentation** folder! This guide outlines how to create, maintain, and organize project documentation so that both end-users and developers have a consistent, high-quality experience. By following these guidelines, we ensure our docs remain accurate, discoverable, and easy to update.

---

## 1. Folder Structure

Our `docs/` directory is divided into:

```plaintext
./docs
├── user_guide/
├── developer_guide/
├── documentation/
└── README.md (this file)
```

### 1.1 `user_guide/`
- **Purpose**: Provide tutorials, how-to guides, and feature explanations for **end-users** (individuals who want to install, run, or configure the software without delving into the code).
- **Examples**:
  - Install, Deploy, Configure, and Run content
  - CLI usage instructions
  - UI walkthroughs
  - Troubleshooting
  - FAQ for non-technical audiences
  - Etc.

### 1.2 `developer_guide/`
- **Purpose**: Share technical documentation intended for **developers** (team members or external contributors) who need to understand the codebase, architecture, and contribution process.
- **Examples**:
  - Architectural overviews and diagrams
  - Code style or linting guidelines
  - API references
  - Internal data flow diagrams
  - Physical hardware data
  - Etc.

### 1.3 `documentation/`
- **Purpose**: Define the project’s documentation standards, best practices, and processes. It includes guidelines on how all docs should be written, updated, and maintained.
- **Examples**:
  - Documentation style guides (voice, tone, structure)
  - Template files for new docs
  - Checklists for doc reviews and acceptance criteria
  - Etc.

---

## 2. Documentation Principles

1. **Clarity & Consistency**
   - Write in a consistent tone and style. Stick to the same tense and voice throughout.
   - When referencing code or commands, use consistent formatting and highlight them with backticks (`) or code blocks.
   - Use plain language. Either write with common terms and or explain acronyms on first use.

2. **Single Source of Truth**
   - Keep information up-to-date in one place. Avoid duplicating content across multiple documents.
   - Link or reference existing docs instead of duplicating and repeating content.

3. **Audience Awareness**
   - Content in `user_guide/` is tailored to end-users who may not understand advanced technical jargon.
   - Content in `developer_guide/` is intended for code-level details, architectural notes, or contribution instructions.

---

## 3. Best Practices

1. **Markdown Standards**
   - Use [Markdown](https://www.markdownguide.org/) for cross-platform readability.
   - Keep line lengths readable (80-120 characters) for easier diffs and reviews.

2. **Consistent Heading Structure**
   - Use top-level headings for major sections (`# Heading`) and nest sub-sections with increasing levels (`##`, `###`, etc.).
   - Start each doc with a short summary and table of contents when it’s longer than a few screens.

3. **Use Plain Language**
   - Whenever possible, use simple language and define acronyms or technical terms on first use.
   - Provide code samples and examples for complex topics.
   - Use relative links to docs and code when relevant.

4. **Cross-Linking**
   - Reference relevant guides across `user_guide/`, `developer_guide/`, and `documentation/`. This prevents fragmentation and helps readers find all the information they need.

5. **Images & Diagrams**
   - Store large diagrams or images in a dedicated folder (e.g., `docs/assets/`).
   - Use descriptive alt text for accessibility.
   - Use [Mermaid diagrams](https://mermaid-js.github.io/mermaid/) for visual clarity.

---

## 4. Expanding Your Documentation

For larger or more complex projects, consider:

- **Versioned docs**: If your project has significant breaking changes or multiple release lines.
- **Auto-generation of API references**: Tools like [rustdoc](https://doc.rust-lang.org/rustdoc/) or other doc generators.
- **Dedicated doc site**: Using [MkDocs](https://www.mkdocs.org/) or [Docusaurus](https://docusaurus.io/) for a static documentation site.

---

If you have suggestions or improvements for these guidelines, please open a Pull Request or reach out to the documentation team.
