## Archivo de configuración de escena

data/scene_description.json

```
{
    "width": 1200,
    "height": 800,
    "samples_per_pixel": 128,
    "max_depth": 50,
    "sky": {
        "texture": "data/skybox.jpg"
    },
    "camera": {
        "look_from": {
            "x": 1.0,
            "y": 0.0,
            "z": 3.0
        },
        "look_at": {
            "x": 0.0,
            "y": 0.0,
            "z": -1.0
        },
        "vup": {
            "x": 0.0,
            "y": 1.0,
            "z": 0.0
        },
        "vfov": 50.0,
        "aspect": 1.5
    },
    "objects": [
        {
            "center": {
                "x": 0.0,
                "y": 6.0,
                "z": 20.0
            },
            "radius": 15.0,
            "material": {
                "Light": {}
            }
        }
    ]
}
```

## Ejemplo de uso

```
$ cargo build --release

$ ./target/release/rust-raytracing  data/scene_description.json output/out.png

Rendering out.png
Frame time: xms
