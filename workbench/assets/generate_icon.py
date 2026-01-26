"""Generate WorkBench-Pro icon as ICO file."""
from PIL import Image, ImageDraw
import math

def create_icon(size):
    """Create a speedometer/gauge icon for WorkBench-Pro."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    center = size // 2
    scale = size / 256  # Base scale on 256px

    # Colors (from theme)
    bg_dark = (15, 52, 96)       # #0f3460
    bg_light = (26, 90, 138)    # #1a5a8a
    accent = (41, 128, 185)     # #2980b9
    white = (255, 255, 255)

    # Background circle
    margin = max(int(8 * scale), 1)
    draw.ellipse([margin, margin, size - margin - 1, size - margin - 1], fill=bg_dark)

    # Inner lighter circle
    inner_margin = max(int(20 * scale), 2)
    draw.ellipse([inner_margin, inner_margin, size - inner_margin - 1, size - inner_margin - 1], fill=bg_light)

    # Gauge arc parameters
    gauge_center_y = center + max(int(8 * scale), 1)
    gauge_radius = max(int(90 * scale), 10)
    arc_width = max(int(12 * scale), 2)

    # Draw gauge arc
    arc_bbox = [
        center - gauge_radius, gauge_center_y - gauge_radius,
        center + gauge_radius, gauge_center_y + gauge_radius
    ]
    if arc_bbox[2] > arc_bbox[0] and arc_bbox[3] > arc_bbox[1]:
        draw.arc(arc_bbox, 135, 405, fill=accent, width=arc_width)

    # Draw tick marks (only on larger sizes)
    if size >= 32:
        tick_length = max(int(12 * scale), 2)
        line_width = max(int(2 * scale), 1)
        for angle in [135, 180, 225, 270, 315, 360, 405]:
            rad = math.radians(angle)
            inner_r = gauge_radius - tick_length // 2
            outer_r = gauge_radius + tick_length // 2
            x1 = center + int(inner_r * math.cos(rad))
            y1 = gauge_center_y + int(inner_r * math.sin(rad))
            x2 = center + int(outer_r * math.cos(rad))
            y2 = gauge_center_y + int(outer_r * math.sin(rad))
            draw.line([(x1, y1), (x2, y2)], fill=(255, 255, 255, 200), width=line_width)

    # Center hub
    hub_radius = max(int(20 * scale), 3)
    draw.ellipse([
        center - hub_radius, gauge_center_y - hub_radius,
        center + hub_radius, gauge_center_y + hub_radius
    ], fill=bg_dark)

    inner_hub = max(hub_radius - max(int(5 * scale), 1), 1)
    if inner_hub > 1:
        draw.ellipse([
            center - inner_hub, gauge_center_y - inner_hub,
            center + inner_hub, gauge_center_y + inner_hub
        ], fill=accent)

    # Needle pointing to ~80% (high performance)
    needle_angle = 330  # degrees
    needle_length = gauge_radius - max(int(8 * scale), 2)
    rad = math.radians(needle_angle)

    needle_tip_x = center + int(needle_length * math.cos(rad))
    needle_tip_y = gauge_center_y + int(needle_length * math.sin(rad))

    # Draw needle as triangle
    needle_width = max(int(6 * scale), 1)
    perp_rad = rad + math.pi / 2

    base1_x = center + int(needle_width * math.cos(perp_rad))
    base1_y = gauge_center_y + int(needle_width * math.sin(perp_rad))
    base2_x = center - int(needle_width * math.cos(perp_rad))
    base2_y = gauge_center_y - int(needle_width * math.sin(perp_rad))

    draw.polygon([(needle_tip_x, needle_tip_y), (base1_x, base1_y), (base2_x, base2_y)], fill=white)

    # Small center dot
    dot_radius = max(int(6 * scale), 1)
    draw.ellipse([
        center - dot_radius, gauge_center_y - dot_radius,
        center + dot_radius, gauge_center_y + dot_radius
    ], fill=white)

    return img


def main():
    # Create icons at multiple sizes for ICO
    sizes = [256, 128, 64, 48, 32, 16]
    images = [create_icon(s) for s in sizes]

    # Save as ICO with multiple sizes
    images[0].save(
        'icon.ico',
        format='ICO',
        sizes=[(s, s) for s in sizes],
        append_images=images[1:]
    )
    print("Created icon.ico")

    # Also save a PNG for web/other uses
    images[0].save('icon.png', format='PNG')
    print("Created icon.png (256x256)")


if __name__ == '__main__':
    main()
