// The download progress bar shown while the browser fetches the app (Trunk's `data-initializer`).
// This runs before the wasm exists, so it can't be Rust; it is the project's only handwritten
// JavaScript. Once the app starts, the Rust side replaces the loading text (and this bar) itself.

const MB = 1024 * 1024;

function part(name) {
  return document.querySelector(`#clerdovector_loading [data-part="${name}"]`);
}

export default function () {
  return {
    onStart() {
      const bar = part("bar");
      if (bar) bar.hidden = false;
    },

    onProgress({ current, total }) {
      const fill = part("fill");
      const count = part("count");
      if (total > 0) {
        const share = Math.min(current / total, 1);
        if (fill) fill.style.width = `${(share * 100).toFixed(1)}%`;
        if (count) count.textContent = `${Math.floor(share * 100)}%`;
      } else if (count) {
        count.textContent = `${(current / MB).toFixed(1)} MB`;
      }
    },

    onFailure(error) {
      const box = document.getElementById("clerdovector_loading");
      if (!box) return;
      box.textContent = "";
      for (const line of [
        `ClerdoVector couldn't load: ${error}`,
        "Check the connection and reload the page.",
      ]) {
        const p = document.createElement("p");
        p.textContent = line;
        box.appendChild(p);
      }
    },
  };
}
