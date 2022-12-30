import numpy as np
import pandas as pd

from vec3 import vec3

def build_image():
    """
    Simple functio to generate a static ppm file.

    To call this and export to a ppm file, run python image_dev.py > {file_name}.ppm
    Then, open the PPM in a viewer to review the image.
    
    """

    # Construct image Specs

    image_width = 256
    image_height = 256

    # Render Image

    print(f"P3\n{image_width} {image_height} \n255\n")

    for j in range(image_height-1, -1, -1):
        # print(f"\rScanlines remaining: {j}")
        for i in range(0, image_width, 1):
            pixel_color = vec3( i / (image_width - 1), j / (image_height - 1), 0.25)
                
            print(f"{pixel_color.x} {pixel_color.y} {pixel_color.z}\n")
    print("\nDone\n")

if __name__ == "__main__":
    build_image()