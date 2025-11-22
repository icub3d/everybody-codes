import matplotlib
matplotlib.use('Agg')  # Use a non-interactive backend
import matplotlib.pyplot as plt

def parse_input(filename):
    """Parses the input file for quest15."""
    with open(filename, 'r') as f:
        content = f.read().strip()
    
    parts = content.split(',')
    instructions = []
    for part in parts:
        direction = part[0]
        distance = int(part[1:])
        instructions.append((direction, distance))
    return instructions

def generate_line_segments(instructions):
    """Generates line segments based on the logic from Map::from_input in the Rust code,
    and returns the walls, the true start point, and the true end point.
    """
    delta = (0, -1) 
    next_pos = (0, 0)
    
    walls = []
    
    true_start_point = next_pos 
    
    for direction, distance in instructions:
        if direction == 'L':
            delta = (delta[1], -delta[0])
        elif direction == 'R':
            delta = (-delta[1], delta[0])
            
        next_pos = (next_pos[0] + delta[0], next_pos[1] + delta[1])
        begin_wall = next_pos
        
        next_pos = (next_pos[0] + delta[0] * (distance - 2), next_pos[1] + delta[1] * (distance - 2))
        end_wall = next_pos
        
        walls.append((begin_wall, end_wall))
        
        next_pos = (next_pos[0] + delta[0], next_pos[1] + delta[1])
        
    true_end_point = next_pos

    return walls, true_start_point, true_end_point

def main():
    """Main function to parse, generate, and plot line segments with a custom theme."""
    # Color palette provided by the user (Catppuccin Mocha)
    catppuccin = {
        'rosewater': '#f5e0dc', 'flamingo': '#f2cdcd', 'pink': '#f5c2e7', 'mauve': '#cba6f7',
        'red': '#f38ba8', 'maroon': '#eba0ac', 'peach': '#fab387', 'yellow': '#f9e2af',
        'green': '#a6e3a1', 'teal': '#94e2d5', 'sky': '#89dceb', 'sapphire': '#74c7ec',
        'blue': '#89b4fa', 'lavender': '#b4befe', 'text': '#cdd6f4', 'subtext1': '#bac2de',
        'subtext0': '#a6adc8', 'overlay2': '#9399b2', 'overlay1': '#7f849c', 'overlay0': '#6c7086',
        'surface2': '#585b70', 'surface1': '#45475a', 'surface0': '#313244', 'base': '#1e1e2e',
        'mantle': '#181825', 'crust': '#11111b'
    }

    input_file = 'ec_2025/src/bin/inputs/quest15-3.txt'
    
    print(f"Reading instructions from '{input_file}'...")
    try:
        instructions = parse_input(input_file)
    except FileNotFoundError:
        print(f"Error: Input file not found at '{input_file}'")
        return
        
    print("Generating line segments and determining start/end points...")
    walls, start_point, end_point = generate_line_segments(instructions)
    
    print("\nPlotting the line segments with the new color theme...")
    fig, ax = plt.subplots(figsize=(10, 10))

    # Set background colors
    fig.patch.set_facecolor(catppuccin['mantle'])
    ax.set_facecolor(catppuccin['base'])

    # Plot line segments
    for start, end in walls:
        ax.plot([start[0], end[0]], [start[1], end[1]], color=catppuccin['blue'], lw=2.0)
        
    # Mark start and end points
    ax.plot(start_point[0], start_point[1], 'o', color=catppuccin['green'], markersize=8, label='Start')
    ax.plot(end_point[0], end_point[1], 'X', color=catppuccin['red'], markersize=8, markeredgewidth=2, label='End')
    
    # Set aspect, title, and labels with colors
    ax.set_aspect('equal', adjustable='box')
    ax.set_title("Line Segments from quest15 part 3", color=catppuccin['text'])
    ax.set_xlabel("X coordinate", color=catppuccin['text'])
    ax.set_ylabel("Y coordinate", color=catppuccin['text'])
    
    # Set grid and ticks colors
    ax.grid(True, color=catppuccin['surface1'], linestyle='--', linewidth=0.5)
    ax.tick_params(axis='x', colors=catppuccin['subtext1'])
    ax.tick_params(axis='y', colors=catppuccin['subtext1'])

    # Set spines colors
    for spine in ax.spines.values():
        spine.set_edgecolor(catppuccin['overlay0'])

    # Set legend colors
    legend = ax.legend()
    legend.get_frame().set_facecolor(catppuccin['surface0'])
    legend.get_frame().set_edgecolor(catppuccin['surface1'])
    for text in legend.get_texts():
        text.set_color(catppuccin['text'])
    
    output_image = 'quest15_p3_line_segments_themed.png'
    try:
        plt.savefig(output_image, facecolor=fig.get_facecolor())
        print(f"Successfully saved themed plot to '{output_image}'")
    except Exception as e:
        print(f"\nError saving plot: {e}")


if __name__ == "__main__":
    main()
