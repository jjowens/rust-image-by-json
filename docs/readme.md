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

### Tests

#### Integration Tests
This application was tested to death. Those tests are available in the tests directory. All JSON files can be found in 
test-assets/test-json.

Each set of tests are based on each function listed above. You can also use those tests as a template. If you go to the workflow tests, 
it has a set of instructions where it can apply a type of filter to your image or a batch of images

### Test Images
The subject of all test images are dogs. Those images were taken from unsplash website. Those images are royalty-free and free to use 
for non-commercial reasons. Most of the pictures are vibrant which make it easier to spot subtle or obvious changes.

