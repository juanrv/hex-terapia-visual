export function processReadingText(rawText: string): string {
  if (rawText.includes("<p>") || rawText.includes("<div>")) {
    const parser = new DOMParser();
    const doc = parser.parseFromString(rawText, "text/html");
    doc
      .querySelectorAll("script, style, link, meta, head")
      .forEach((el) => el.remove());
    doc.querySelectorAll("*").forEach((el) => {
      el.removeAttribute("class");
      el.removeAttribute("style");
      el.removeAttribute("id");
    });
    return doc.body.innerHTML;
  }

  const escaped = rawText
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
  return escaped
    .split(/\n\n+/)
    .filter((p) => p.trim() !== "")
    .map((p) => `<p>${p.replace(/\n/g, "<br>")}</p>`)
    .join("\n");
}
