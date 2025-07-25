<?php
/**
 * Basic OCR example using the PHP-OCRS extension
 *
 * This example demonstrates how to use the PhpOcrEngine class to perform
 * basic OCR on an image file.
 */

// Check if the extension is loaded
if (!extension_loaded('php-ocr')) {
    die("The php_ocr extension is not loaded. Please check your PHP configuration.\n");
}

// Check if an image path was provided
if ($argc < 2) {
    echo "Usage: php basic_ocr.php <image_path>\n";
    echo "Example: php basic_ocr.php test.jpg\n";
    exit(1);
}

$image_path = $argv[1];


// Check if the image file exists
if (!file_exists($image_path)) {
    die("Error: The image file '$image_path' does not exist.\n");
}

try {
    // Create a new OCR engine instance
    echo "Creating OCR engine...\n";
    $ocr = new PhpOcrEngine("../models/text-detection.rten",
    "../models/text-recognition.rten");



    // Process the image
    echo "Processing image: $image_path\n";
    $start_time = microtime(true);

    $text = $ocr->processImage($image_path);

    $end_time = microtime(true);

    $processing_time = round($end_time - $start_time, 2);


    // Print the results
    echo "\nProcessing completed in $processing_time seconds.\n\n";
    echo "Recognized text:\n";
    echo "----------------------------------------\n";
    echo $text;
    echo "\n----------------------------------------\n";

    // Print some statistics
    $word_count = str_word_count($text);
    $char_count = strlen($text);

    echo "\nStatistics:\n";
    echo "Word count: $word_count\n";
    echo "Character count: $char_count\n";

} catch (OCREngineException $e) {
    echo "Error: " . $e->getMessage() . "\n";
    exit(1);
}