import numpy as np
import pandas as pd


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

    for num in range(image_height-1, -1, -1):
        print(f"\rScanlines remaining: {num}")
        for secnum in range(0, image_width, 1):
            r = float(secnum)/ (image_width - 1)
            g = float(num) / (image_height - 1)
            b = 0.25

            ir = int(255.999 * r)
            ig = int(255.999 * g)
            ib = int(255.999 * b)

            print(f"{ir} {ig} {ib}\n")
    print("\nDone\n")

if __name__ == "__main__":
    build_image()