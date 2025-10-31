import init, { Renderer } from '../pkg/renderer.js';
let isDragging = false, isInCanvas = false;
// Load the WASM module
await init();
const renderer = await new Renderer();
await load_3();
// set default camera values here

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
// -------------------------------- canvas ------------------------------------------
const canvas_ = document.getElementById("canvas");
const rect = canvas.getBoundingClientRect();
let width = canvas.width;
let height = canvas.height;
// Mouse down (start drag)
canvas_.addEventListener("mousedown", (e) => {
    if (e.button === 0) { // left button
        isDragging = true;
       
    }

});
// Mouse move (if dragging, update rotation)
canvas.addEventListener("mousemove", (e) => {
    let x = e.clientX - rect.left;
    let y = e.clientY - rect.top;
    if (isDragging) {
        // camera update happens here
        console.log("rotating model");
        let ndcX = (x / width) * 2.0 - 1.0;
        let ndcY = 1.0 - (y / height) * 2.0;
        console.log("ndcX: ", ndcX, "ndcY: ", ndcY)
    }
});
// Mouse up (stop drag)
canvas.addEventListener("mouseup", () => {
    isDragging = false;
});
// Also stop if mouse leaves canvas while dragging
canvas.addEventListener("mouseleave", () => {
    isDragging = false;
    isInCanvas = false;
});
canvas.addEventListener("mouseenter", () => {
    isInCanvas = true;
});

// Mouse wheel scroll
// Mouse wheel scroll
canvas.addEventListener("wheel", (e) => {
    if (isInCanvas) {
        e.preventDefault(); // <-- stop the page from scrolling

        if (e.deltaY < 0) { // scroll up
            console.log("zoomed in");
        } else if (e.deltaY > 0) { // scroll down
            console.log("zoomed out");
        }
    }
}, { passive: false }); // IMPORTANT: allows preventDefault

// -------------------------------- canvas ------------------------------------------


// renderer
async function load_3() {
    // fetch  the model
    //const response = await fetch("models/t_rex_model_glb/source/trex.glb");
    const response = await fetch("models/flightHelmet/flightHelmet.glb");
    const bytes = new Uint8Array(await response.arrayBuffer());
    const result = await renderer.render(bytes);
}
