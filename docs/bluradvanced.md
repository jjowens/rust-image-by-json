# Blur Advanced

This instruction will add a blurring effect as well as different types of blurring

| Name     | Data Type | Accepted Parameter(s)                                                    | Minimum Value | Maximum Value | Description                                                        |
|----------|-----------|--------------------------------------------------------------------------|---------------|---------------|--------------------------------------------------------------------|
| process  | String    | unsharpen                                                                | NA            | NA            | Set type of process                                                |
| blurtype | String    | anisotropic, kernel, radius, sigma, smooth_3, smooth_5, smooth_7, kernel | 0             | 1,000         | Set type of blur type                                              |
| value    | String    | NA                                                                       | 0             | 1,000         | Set level of blurring                                              |
|     |     |                                                                        | [0,0]         | [1000,1000]   | Anisotropic only. Set level of blurring as an array|


## Examples

## Set blur type to anisotropic. Set array to [10,10]

Instructions: Set blur type to anisotropic. Set array to [10,10]
````
{"process": "bluradvanced", "blurtype": "anisotropic", "value": "10,10"}
````

| Original Image                     | Updated Image                          |
|------------------------------------|----------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/bluradvanced-anisotropic-10-10/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/bluradvanced-anisotropic-10-10/dog2.png) |

## Set blur type to kernel. Set value to 10

Instructions: Set blur type to kernel. Set value to 10
````
{"process": "bluradvanced", "blurtype": "kernel", "value": "10"}
````

| Original Image                     | Updated Image                          |
|------------------------------------|----------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/bluradvanced-kernel/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/bluradvanced-kernel/dog2.png) |

## Set blur type to sigma. Set value to 10

Instructions: Set blur type to sigma. Set value to 10
````
{"process": "bluradvanced", "blurtype": "sigma", "value": "10"}
````

| Original Image                     | Updated Image                          |
|------------------------------------|----------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/bluradvanced-sigma-10/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/bluradvanced-sigma-10/dog2.png) |

## Set blur type to smooth_3

Instructions: Set blur type to smooth_3.
````
{"process": "bluradvanced", "blurtype": "smooth_3", "value": "smooth_3"}
````

| Original Image                     | Updated Image                          |
|------------------------------------|----------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/bluradvanced-smooth-3/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/bluradvanced-smooth-3/dog2.png) |

## Set blur type to smooth_5

Instructions: Set blur type to smooth_5.
````
{"process": "bluradvanced", "blurtype": "smooth_5", "value": "smooth_5"}
````

| Original Image                     | Updated Image                                   |
|------------------------------------|-------------------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/bluradvanced-smooth-5/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/bluradvanced-smooth-5/dog2.png) |

## Set blur type to smooth_7

Instructions: Set blur type to smooth_7.
````
{"process": "bluradvanced", "blurtype": "smooth_7", "value": "smooth_7"}
````

| Original Image                     | Updated Image                                   |
|------------------------------------|-------------------------------------------------|
| ![image](images/original/dog1.png) | ![image](images/bluradvanced-smooth-7/dog1.png) |
| ![image](images/original/dog2.png) | ![image](images/bluradvanced-smooth-7/dog2.png) |


