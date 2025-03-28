document.addEventListener("alpine:init", () => {
    Alpine.prefix("data-x-");

    Alpine.store("strife", window.initialStore?.Results[0] || { });

    window.strife.subscribe((data) => {
        Object.assign(Alpine.store("strife"), data);
    });
    
});