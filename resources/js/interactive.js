
$(document).ready(function () {
    let svg = d3.select('svg');
    let visualizer_root = svg.append("g");

    let width = Math.min(CONTENT_WIDTH, window.innerWidth);
    let height = Math.min(SVG_HEIGHT, window.innerHeight);
    let defaultWord = "crayon";

    let nameRenderer = new MountainsAndLakes(visualizer_root, width/2, height/2);
    nameRenderer.render(defaultWord);

    let mal_input = $("#mal-input");
    mal_input.on("keyup", function(e) {
        let current = $(this).val();
        let word = current.replace(/[^a-zA-Z]/gi, '').toLowerCase();
        $(this).val(word);

        if (word == "") {
            nameRenderer.render(defaultWord);
        } else {
            nameRenderer.render(word);
        }
    });
});
