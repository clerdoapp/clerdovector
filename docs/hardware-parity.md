# Hardware parity

> **Last reviewed:** 2026-10-10 · **Last updated:** 2026-11-01 · **Change:** patch (Linux: on a Wayland session with a pen attached, Automatic opens the window through XWayland, where the pen draws, #4967) · **Target:** Adobe Illustrator 2026 (30.x)

What hardware Illustrator uses, per platform, against ClerdoVector. Illustrator's side is from its public
documentation (GPU Performance, touch workspace, pen pressure pages); ours from the code
(`crates/ui-egui/src/canvas.rs`, `touch.rs`, `graphics.rs`, `crates/render`). Part of
[target-app-parity.md](target-app-parity.md).

## Summary

**~35% (estimated), 25–45 h.** ClerdoVector rasterizes the canvas on the CPU (multithreaded `vello_cpu`) and uses
the GPU only to composite, so any GPU that can show a window works, at the cost of GPU-accelerated preview
on huge documents. ClerdoVector reads pen pressure on Windows, on the web, on macOS, and on Linux under X11 and XWayland, and it
reads the pen's eraser end on macOS, on Linux under X11 and XWayland, and on the web when it presses.
reads the pen's eraser end on Windows, on macOS and on Linux under X11 and XWayland.

| Feature | Illustrator | ClerdoVector macOS | Windows | Linux | Web | Status / hours |
|---|---|---|---|---|---|---|
| GPU-accelerated canvas (GPU Performance, GPU Preview, GPU anti-aliasing) | Metal, DirectX | CPU raster, GPU composite (wgpu/Metal) | CPU raster, GPU composite (DX12, OpenGL fallback) | CPU raster, GPU composite (Vulkan/GL) | WebGPU, WebGL2 fallback | partial; a GPU renderer spike is pending (M5): 15–25 h |
| Multithreaded rendering off the UI thread | partial | yes | yes | yes | single-threaded | beyond Illustrator |
| Hybrid-graphics GPU choice | automatic | power-saving GPU by default, Preferences › Graphics Processor | same; DX12 then OpenGL; no Vulkan unless asked (#545, #806) | the desktop's GPU | browser | done |
| Restart on another GPU when the first fails to present | — | yes (#502, #964) | yes | yes | restart on lost device (#369) | done |
| Pen pressure | yes (Wacom, Windows Ink, macOS tablet events) | yes, read from AppKit's tablet events before the window gets them (winit reports none), each move of a stroke with its own pressure, mouse event coalescing off (#852; not yet tried on a Mac) | yes (pointer force via `WM_POINTER`) | under X11 and XWayland (Preferences › Performance › Linux Display Server, #1151, #2028; Automatic uses XWayland when a pen is attached on a Wayland session, #4967), read from XInput2 raw events (#764); **no** on native Wayland, where winit reports none and the pen may not reach the window (#491) | browser pointer events, with coalesced events; Apple Pencil on iPad (hover, palm rejection: fingers navigate after a pen contact until reload) | partial: 2–4 h for Wayland (winit 0.31), plus a check on a Mac |
| Pen tilt, bearing, barrel rotation (6D Art Pen) | yes, drives brushes | no | no | no | no | missing: 4–8 h after the input layer (#372) |
| Pen eraser end | yes | the eraser end (AppKit's tablet proximity events) selects the Eraser tool and the tip brings back the tool used before, as on Linux (#852; not yet tried on a Mac) | no | under X11 and XWayland, the eraser end selects the Eraser tool and the tip brings back the tool used before (#764); **no** on native Wayland | a press of the eraser end (Pointer Events eraser button, `button` 5) selects the Eraser tool, and that press erases; a press of the tip brings back the tool used before; Pointer Events report no end while the pen hovers | partial |
| Pen eraser end | yes | the eraser end (AppKit's tablet proximity events) selects the Eraser tool and the tip brings back the tool used before, as on Linux (#852; not yet tried on a Mac) | with Windows Ink, an inverted pen or its eraser button (`PEN_FLAG_INVERTED`, `PEN_FLAG_ERASER` in the pen's pointer messages) selects the Eraser tool and the tip brings back the tool used before, as on Linux (not yet tried with a pen) | under X11 and XWayland, the eraser end selects the Eraser tool and the tip brings back the tool used before (#764); **no** on native Wayland | no | partial |
| Touch gestures (pinch zoom, two-finger pan, rotate view, Touch workspace) | yes (Windows touch, Surface) | trackpad pinch, scroll and rotate | pinch, pan (a second finger cancels the first finger's tool press, #4198); two-finger tap undo, three-finger redo (#585) | same where winit reports touch | eframe until the first pen contact, then Pointer Events (pen and fingers told apart); long press opens the context menu | partial: touch workspace and touch-screen rotation missing, 3–5 h |
| Trackpad (smooth scroll, pinch) | yes | yes | precision touchpads | yes | yes | done |
| HiDPI / Retina, per-monitor scale | yes | yes | yes | yes | yes | done; a window dragged between monitors of different scale can still grow on Windows (#1459, open); the custom title bar's window on Windows stays truly borderless so the pointer lines up with what is drawn (#1791, #1758; awaiting the reporter's confirmation) |
| Multiple monitors: panels and document windows on another screen | yes | no (one OS window) | no | no | — | missing: 6–10 h (with multiple windows, gap G7) |
| Display colour profile (proof and display through the monitor's ICC) | yes | no | no | no | — | missing: 3–5 h |
| 3D GPU rendering (Materials, ray tracing) | yes | — | — | — | — | blocked on 3D (gap G3) |
