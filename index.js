import("./pkg")
  .then((wasm) => {
    const seed = document.getElementById("seed");
    const seedDisplay = document.getElementById("seed-display");
    
    seed.addEventListener("input", () => {
      console.log(seed.value);
      const integer = parseInt(seed.value) || 1;
      seedDisplay.innerHTML = seed.value;
      wasm.start(integer);
    });
    wasm.start(1);
  })
  .catch(console.error);
