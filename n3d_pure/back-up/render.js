import init, { Renderer } from '../pkg/renderer.js';

await init();
const renderer = await new Renderer();
// resizing window
let resizeTimer;
window.addEventListener("resize", async () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
        console.log("Resize finished!");
        console.log("New size:", window.innerWidth, "x", window.innerHeight);
    }, 300); // 300ms after last resize event
    //await renderer.reconfigure_surface();
});

// renderer
async function load_3() {
    // fetch  the model
    const response = await fetch("models/t_rex_model_glb/source/trex.glb");
    const bytes = new Uint8Array(await response.arrayBuffer());
    const result = await renderer.render(bytes);
}
load_3();

