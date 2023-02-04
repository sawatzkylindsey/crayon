
class MountainsAndLakes {
    constructor(root, offset_x, offset_y) {
        this.root = root;
        this.offset_x = offset_x;
        this.offset_y = offset_y;
    }

    render(word) {
        // Clear the existing drawing.
        $(".drawing").remove();

        let stroke_width = 1;
        let offset_x = this.offset_x - (MountainsAndLakes.SIZE / 2);
        let offset_y = this.offset_y - (stroke_width / 2);
        this.root.append("line")
            .attr("class", "drawing")
            .attr("x1", offset_x - 1)
            .attr("y1", offset_y)
            .attr("x2", offset_x + MountainsAndLakes.SIZE + 1)
            .attr("y2", offset_y)
            .attr("stroke-width", stroke_width)
            .attr("stroke", "#AA7942");

        var alpha_x = function (letter) {
            var l = "abcdefghijklmnopqrstuvwxyz".indexOf(letter);
            console.assert(l >= 0, "invalid letter '" + letter + "'.");
            // Scale the maximum size across the alphabet (there are 25 gaps between the 26 letters).
            return (MountainsAndLakes.SIZE / 25) * l;
        };

        var mountain_interpolation = d3.interpolateLab("#33691e", "#85CE86");
        var lake_interpolation = d3.interpolateLab("#0d47a1", "#4192d9");

        for (var index = 1; index < word.length; index++) {
            // The mode is -1 for mountains, and +1 for lakes.
            // We'll use this to determine whether to add or subtract to the center line along the y-axis.
            let mode = (((index - 1) % 2) * 2) - 1;

            let x1 = alpha_x(word[index - 1]);
            let x2 = alpha_x(word[index]);
            let x_delta = Math.abs(x2 - x1);
            // Calculate the y_delta, which is like an inverse of the x_delta.
            // So, the largest x_delta makes the smallest y_delta, and the smallest x_delta makes the largest y_delta.
            let slope = ((-x_delta) / MountainsAndLakes.SIZE) + 1;
            let y_delta;
            var colour;

            // We want mountains to get higher than lakes.
            // We want green mountains and blue lakes.
            if (mode == -1) {
                y_delta = slope * MountainsAndLakes.SIZE;
                colour = mountain_interpolation((index - 1) / (word.length - 1));
            } else {
                y_delta = slope * (MountainsAndLakes.SIZE / 2);
                colour = lake_interpolation((index - 1) / (word.length - 1));
            }

            let path = d3.path();
            path.moveTo(offset_x + x1, offset_y);
            path.quadraticCurveTo(offset_x + Math.min(x1, x2) + (x_delta / 2), offset_y + (mode * y_delta),
                offset_x + x2, offset_y);
            path.closePath();

            // Make each added mountain/lake a little more transparent.
            let opacity =  1 - ((index - 1) / (word.length));
            this.root.append("path")
                .attr("class", "drawing")
                .attr("d", path)
                .attr("fill", colour)
                .attr("opacity", opacity)
                .attr("stroke", colour);
        }
    }
}

MountainsAndLakes.SIZE = 300;
