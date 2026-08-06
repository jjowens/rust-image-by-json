# Brighten

This instruction will add a brightening effect

| Name    | Data Type | Accepted Parameter(s) | Minimum Value | Maximum Value | Description              |
|---------|-----------|-----------------------|---------------|---------------|--------------------------|
| process | String    | brighten              | NA            | NA            | Set type of process      |
| value   | String    | NA                    | 0             | 1,000         | Set level of brightening |


## Examples

## Brighten image to 10

Instructions: Set brighten to 10
````
{"process": "brighten", "value": "10"}
````

| Original Image                     | Updated Image                         |
|------------------------------------|---------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/brighten-10/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/brighten-10/dog2.png) |

## Brighten image to 100

Instructions: Set brighten to 100
````
{"process": "brighten", "value": "100"}
````

| Original Image                     | Updated Image                          |
|------------------------------------|----------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/brighten-100/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/brighten-100/dog2.png) |