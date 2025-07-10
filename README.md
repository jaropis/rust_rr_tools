# rust_rr_tools

A Rust-based tool for processing QRS complex position files and converting them to RR interval format with annotations. This software reads text files containing QRS complex positions and annotations, then exports the data as RR intervals with corresponding annotations in a standardized format.

## Overview

This tool is designed for cardiovascular signal processing, specifically for converting raw QRS complex detection data into RR interval measurements. RR intervals represent the time between consecutive R-peaks in an ECG signal and are fundamental for heart rate variability (HRV) analysis.

## Features

- **File Format Conversion**: Converts various input formats to standardized RR interval output
- **Annotation Processing**: Handles different types of beat annotations (Normal, Ventricular, Supraventricular, etc.)
- **Differential Processing**: Can compute RR intervals as differences between consecutive QRS positions
- **Outlier Correction**: Optional algorithm to detect and correct physiologically implausible RR intervals
- **Batch Processing**: Processes all files with specified extension in the current directory
- **Sampling Rate Normalization**: Supports conversion between different sampling rates

## Input File Format

The input files should contain QRS complex data with the following structure:

- Each line represents one QRS complex
- First column: QRS position (in samples or time units)
- Second column: Beat annotation (N=Normal, V=Ventricular, S=Supraventricular, other=Unknown)

Example input file:

```
1250 N
2480 N
3720 V
4950 N
6180 N
```

## Output File Format

The output files contain RR intervals with annotations:

```
RR	annot
1230	0
1240	1
1230	1
1230	0
```

Where:

- `RR`: RR interval duration (in milliseconds or specified units)
- `annot`: Numerical annotation (0=Normal, 1=Ventricular, 2=Supraventricular, 3=Other)

## Command Line Usage

```bash
./rust_rr_tools <input_ext> <output_ext> <multiplier> <diff> <skip> <scout> <correct> [sampling_rate]
```

### Parameters

1. **input_extension** (string): File extension of input files to process (e.g., "rri", "txt")
2. **output_extension** (string): File extension for output files (e.g., "rea")
3. **rr_multiplier** (float): Scaling factor for RR intervals (e.g., 1000 to convert seconds to milliseconds)
4. **diff** (boolean): Whether to compute differences between consecutive QRS positions
   - `true`: Calculate RR intervals as differences (position[i] - position[i-1])
   - `false`: Use raw values from input
5. **skip** (integer): Number of initial beats to skip (useful for removing artifacts at recording start)
6. **scout** (boolean): Scout mode - process files but don't write output (useful for testing)
7. **correct** (boolean): Enable outlier correction algorithm
   - `true`: Apply physiological outlier detection and correction
   - `false`: No correction applied
8. **sampling_rate** (float, optional): Sampling rate for normalization (Hz)

### Processing Modes

#### Differential Mode (`diff=true`)

- Calculates RR intervals as the difference between consecutive QRS positions
- First RR interval is skipped as it cannot be calculated
- Useful when input contains absolute QRS positions

#### Direct Mode (`diff=false`)

- Uses input values directly as RR intervals
- Useful when input already contains pre-calculated intervals

#### Outlier Correction (`correct=true`)

When enabled, the correction algorithm:

1. Calculates the average RR interval for all beats annotated as "Normal" (flag 0)
2. Identifies outliers: RR intervals with non-normal annotations that exceed 5× the normal average
3. Replaces outliers with the calculated normal average
4. Reports the number of corrections made

This helps remove physiologically implausible values that may result from detection errors.

## Examples

### Example 1: RRI Processing with Correction

```bash
./rust_rr_tools "rri" "rea" 1000 true 3 false true 1024
```

### RRI

```bash
./rust_rr_tools "rri" "rea" 1000 true 3 false false 1024
cargo run "rri" "rea" 1000 true 3 false false 1024
```

### MASTER

```bash
./rust_rr_tools "txt" "rea" 1000 true 1 true false
cargo run "txt" "rea" 1000 false 1 true false
```

- Process files with `.rri` extension
- Output files with `.rea` extension
- Multiply RR intervals by 1000 (seconds to milliseconds)
- Use differential mode (calculate differences)
- Skip first 3 beats
- Normal processing mode (not scout)
- Enable outlier correction
- Sampling rate: 1024 Hz

### Example 2: Master File Processing

```bash
./rust_rr_tools "txt" "rea" 1000 false 1 false false
```

- Process `.txt` files
- Use direct mode (no differences)
- Skip first beat only
- No outlier correction
- No sampling rate specified

### Example 3: Scout Mode (Testing)

```bash
cargo run "rri" "rea" 1000 true 3 true false
```

- Scout mode enabled - analyze files but don't write output
- Useful for verifying parameters before actual processing

## Building and Running

### Prerequisites

- Rust compiler (https://rustup.rs/)

### Building

```bash
cargo build --release
```

### Running

```bash
# Using cargo
cargo run -- <parameters>

# Using compiled binary
./target/release/rust_rr_tools <parameters>
```

## Output Information

The program provides detailed feedback including:

- List of processed files
- Argument parsing confirmation
- Unique annotations found across all files
- Correction statistics (when correction is enabled):
  - Average RR interval for normal beats
  - Correction threshold (5× average)
  - Number of outliers corrected

## Technical Details

### Annotation Mapping

The software maps textual beat annotations to numerical codes:

- `N` (Normal) → `0`
- `V` (Ventricular) → `1`
- `S` (Supraventricular) → `2`
- All others → `3`

### Sampling Rate Handling

When a sampling rate is provided, RR intervals are normalized:

```
normalized_rr = (raw_interval * multiplier) / sampling_rate
```

### Error Handling

- Graceful handling of file read errors
- Automatic skipping of files that don't match the input extension
- Validation of numerical parameters
- Warning messages for correction edge cases

## Use Cases

- **Heart Rate Variability Analysis**: Convert raw ECG annotations to HRV-compatible format
- **Arrhythmia Research**: Process large datasets with automatic outlier correction
- **Signal Quality Assessment**: Use scout mode to evaluate data before processing
- **Multi-format Conversion**: Standardize data from different acquisition systems
