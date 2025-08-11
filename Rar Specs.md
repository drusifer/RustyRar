# **Verified RAR 5.0 Binary Format Specification**

Version: 1.0  
Status: Verified

### **1\. Introduction**

This document provides a technical specification for the RAR 5.0 archive format. The format is a proprietary, block-based system designed for data compression and archiving. A RAR file is constructed as a sequence of variable-length blocks, each serving a specific purpose, such as defining archive-wide attributes, describing a file, or holding service data.

The specification focuses on the **RAR 5.0 format**, which was introduced with WinRAR 5.0 and is not backward compatible with previous versions (e.g., RAR 4.x).

### **2\. Overall Archive Structure**

A RAR 5.0 archive begins with a fixed 8-byte signature, followed by a series of blocks. The most fundamental blocks are the **Main Archive Header** and one or more **File Headers**.

The logical layout is as follows:

1. **RAR Signature:** A fixed marker identifying the file as a RAR 5.0 archive.  
2. **Main Archive Header:** Contains global information about the archive (e.g., volume information, archive-level flags). This block is encrypted if the headers are encrypted.  
3. **File Header & Data:** A block describing a single file or directory, immediately followed by the compressed data for that file. This pair can repeat for every file in the archive.  
4. **(Optional) Service Headers:** Contains supplementary data, such as a Quick Open record.  
5. **End of Archive Header:** A special service header that marks the final block in the archive.

#### **Mermaid Diagram of Archive Structure**

graph TD  
    subgraph RAR Archive  
        A\[RAR Signature \<br\> 8 bytes\] \--\> B(Main Archive Header);  
        B \--\> C{File Block 1};  
        C \--\> D\[File Header 1\];  
        D \--\> E\[Compressed Data 1\];  
        E \--\> F{...};  
        F \--\> G{File Block N};  
        G \--\> H\[File Header N\];  
        H \--\> I\[Compressed Data N\];  
        I \--\> J(End of Archive Header);  
    end

### **3\. General Block Structure**

Every block in a RAR 5.0 archive (except the initial signature) is preceded by a generic header that defines its type, flags, and size.

| Field | Data Type | Size (Bytes) | Description |
| :---- | :---- | :---- | :---- |
| **CRC32** | uint32\_t | 4 | A checksum of the block header (from Header Size to the end of the block-specific fields). |
| **Header Size** | vint | 1-4 | The total size of the block header. A variable-length integer. |
| **Header Type** | vint | 1-4 | An integer identifying the type of block (e.g., 1 for Main Archive, 2 for File). |
| **Header Flags** | vint | 1-4 | A bitmask of flags. If the 0x0001 bit is set, a Data Size field follows. |
| **Data Size** | vint | 1-4 | **(Optional)** The size of the data area that follows this header (e.g., compressed file size). |
| **Extra Data** | byte\[\] | Variable | **(Optional)** Additional block-specific fields defined by Header Flags. |

**Variable-Length Integers (vint):** RAR 5.0 uses a variable-length encoding for many integer fields to save space. Each byte uses its 7 lower bits for data, and the most significant bit (MSB) acts as a continuation flag. If the MSB is 1, the next byte is also part of the integer. If it is 0, this is the last byte.

### **4\. Key Block Types**

#### **4.1. RAR Signature (Marker Block)**

This is a fixed, un-headed block that must be the first 8 bytes of the file.

* **Signature:** 0x52 0x61 0x72 0x21 0x1A 0x07 0x01 0x00  
* **ASCII:** Rar\!....

#### **4.2. Main Archive Header**

* **Header Type:** 1  
* **Description:** Contains global properties for the entire archive. It must be the first block after the signature.

**Block-Specific Fields:**

| Field | Data Type | Description |
| :---- | :---- | :---- |
| **Archive Flags** | vint | Bitmask of archive properties (e.g., 0x0001 for Volume, 0x0002 for Solid). |
| **Volume Number** | vint | **(Optional)** The sequence number of the volume, starting from 0\. Present if Archive Flag 0x0001 is set. |
| **Locator Record** | byte\[\] | **(Optional)** Service data for quick archive updates. Present if Archive Flag 0x0010 is set. |

#### **4.3. File Header**

* **Header Type:** 2  
* **Description:** The most common block, providing metadata for a single file or directory.

**Block-Specific Fields:**

| Field | Data Type | Description |
| :---- | :---- | :---- |
| **File Flags** | vint | Bitmask of file properties (e.g., 0x0001 for Directory, 0x0002 for Unix timestamp). |
| **Unpacked Size** | vint | The original, uncompressed size of the file. |
| **File Attributes** | vint | Host OS-specific file attributes (e.g., MS-DOS attributes on Windows). |
| **Modification Time** | uint32\_t | **(Optional)** 32-bit Unix timestamp. Present if File Flag 0x0002 is set. |
| **File CRC32** | uint32\_t | **(Optional)** A checksum of the uncompressed file data. Not present for directories. |
| **Compression Info** | vint | Contains compression algorithm and host OS. |
| **File Name Length** | vint | The length of the File Name field in bytes. |
| **File Name** | byte\[\] | The file name, encoded in UTF-8. |
| **(Optional) Symlink** | byte\[\] | If the file is a symbolic link, this contains the target path. |

#### **4.4. End of Archive Header**

* **Header Type:** 5  
* **Description:** Marks the end of the archive. Any data following this block should be ignored.

**Block-Specific Fields:**

| Field | Data Type | Description |
| :---- | :---- | :---- |
| **End Archive Flags** | vint | Bitmask of properties. 0x0001 indicates the archive is not the last volume in a multi-volume set. |

### **5\. Data Integrity and Encryption**

* **CRC32:** All headers and uncompressed file data are protected by CRC32 checksums to detect corruption.  
* **Encryption:** RAR 5.0 uses AES-256 in CBC mode for encryption. If header encryption is enabled, all blocks after the RAR Signature are encrypted, obscuring the archive's structure and contents without the correct password. The encryption parameters are derived from the password using a PBKDF2 function with a specified salt.