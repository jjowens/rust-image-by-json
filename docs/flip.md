# Flip

This instruction will flip images horizontally, vertically, or both. You can use both by running instructions one after the other.


| Name    | Data Type | Accepted Parameter(s) | Minimum Value | Maximum Value | Description                           |
|---------|-----------|-----------------------|---------------|---------------|---------------------------------------|
| process | String    | flip                  | NA            | NA            | Set type of process                   |
| value   | String    | h or v                | NA            | NA            | Flip image horizontally or vertically |

## Examples

## Flip H

Instructions: Flip image horizontally
````
{"process": "flip", "value": "h"}
````

| Original Image                     | Updated Image                    |
|------------------------------------|----------------------------------|
| ![image](images/original/dog1.png) | ![image](images/flip-h/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/flip-h/dog2.png) |

## Flip V

Instructions: Flip image vertically
````
{"process": "flip", "value": "v"}
````

| Original Image                     | Updated Image                    |
|------------------------------------|----------------------------------|
| ![image](images/original/dog1.png) | ![image](images/flip-v/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/flip-v/dog2.png) |

## Flip H and V

Instructions: Flip image horziontally and vertically
````
{"process": "flip", "value": "h"},
{"process": "flip", "value": "v"}
````

| Original Image                     | Updated Image                          |
|------------------------------------|----------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/flip-h-and-v/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/flip-h-and-v/dog2.png) |