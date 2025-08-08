# php-ocr

A PHP extension for Optical Character Recognition (OCR) built in Rust. This extension leverages the [ocrs](https://github.com/robertknight/ocrs) library to provide high-performance text detection and recognition capabilities directly within PHP applications.

## Features

- Fast text detection and recognition
- Simple PHP API
- Built with Rust for performance and safety
- Uses pre-trained models for text detection and recognition

## Requirements

- Rust toolchain (rustc, cargo)
- PHP development headers
- PHP 7.4 or newer

## Building the Extension

To build the extension, you need to have Rust installed on your system. If you don't have Rust installed, you can install it using [rustup](https://rustup.rs/).

Once Rust is installed, you can build the extension using cargo:

```bash
# Clone the repository
git clone https://github.com/federicoemartinez/php-ocr.git
cd php-ocr

# Build the extension in release mode
cargo build --release
```

After a successful build, the extension library `libphp_ocr.so` will be available in the `target/release` directory.

## Installation

To use the extension with PHP, you need to:

1. Copy the built library to your PHP extensions directory or a directory of your choice
2. Enable the extension in your `php.ini` file:

```ini
extension=libphp_ocr.so
```

You can verify that the extension is loaded by running:

```bash
php -m | grep php-ocr
```

## Usage

The extension provides a `PhpOcrEngine` class that you can use to perform OCR on images. Here's a basic example:

```php
<?php
// Check if the extension is loaded
if (!extension_loaded('php-ocr')) {
    die("The php_ocr extension is not loaded. Please check your PHP configuration.\n");
}

try {
    // Create a new OCR engine instance with the paths to the model files
    $ocr = new PhpOcrEngine("path/to/text-detection.rten", 
                            "path/to/text-recognition.rten");

    // Process an image
    $text = $ocr->processImage("path/to/image.jpg");

    // Print the recognized text
    echo "Recognized text:\n";
    echo $text;

} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
```

For a more complete example, see [examples/basic_ocr.php](examples/basic_ocr.php).

## Models

The extension requires two model files:
- A text detection model (`text-detection.rten`)
- A text recognition model (`text-recognition.rten`)

These models are included in the `models` directory of this repository and come from [ocrs-models](https://github.com/robertknight/ocrs-models)

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

## Acknowledgements

This extension uses the [ocrs](https://github.com/robertknight/ocrs) library by Robert Knight for OCR functionality and [ext-php-rs](https://github.com/davidcole1340/ext-php-rs)
