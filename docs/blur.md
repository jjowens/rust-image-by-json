# Blur

This instruction will add a blurring effect

| Name    | Data Type | Accepted Parameter(s) | Minimum Value | Maximum Value | Description           |
|---------|-----------|-----------------------|---------------|---------------|-----------------------|
| process | String    | blur                  | NA            | NA            | Set type of process   |
| value   | String    | NA                    | 0             | 1,000         | Set level of blurring |


## Examples

Instructions: Set blur to 10
````
{"process": "blur", "value": "10"}
````

| Original Image                     | Updated Image                     |
|------------------------------------|-----------------------------------|
| ![image](images/original/dog1.png) | ![image](images/blur-10/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/blur-10/dog2.png) |