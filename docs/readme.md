# rust-image-by-json - Documentation

### What does rust-image-by-json do?

It can open a json file and read instructions in sequence to process images; singular or by batch. It will export images to a separate folder
to prevent overwriting original images.

### Available Functions

- HueRotate
- Rotate
- Unsharpen
- Blur. There are varied options to set different types of blurring
- Resize images by pixels or percentage
- Resave images in different formats
- Brighten and Contrast images
- Create grayscale images
- Flip images horizontally, vertically, or both

### Integration Tests
This application was tested to death. Those tests are available in the tests directory. All JSON files can be found in 
test-assets/test-json.