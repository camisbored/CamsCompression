# CamsCompression
A custom compression algorithm implemented in Rust.

Noticing how often padding of the same byte (0x00, 0xFF) occured in large segments of files, the idea for this algorithm 
arose as a way to see how much of a difference removing just those sequences would make. While the practical use for this exists with only
rather specific cases, it does prove as a great exercise in understanding compression algorithms. The best case and average case for this algorithm
can be seen in the following image, as well as some timing and other metrics.

![image](https://github.com/camisbored/CamsCompression/assets/81730723/b00f9cde-e377-49b8-8622-84b9241f4dde)


Future implementations of the algorihtm itself may involve looking for more sophisitcated patterns, which I believe will actually turn this into a 
powerful tool with more real world use cases. Recently beginning to learn rust, I thought this would be a great opporitunity to see how the language
handles for a program like this. The answer was surprisingly well. This supports files of up to 4gb, and was able to handle with no memory issues.

The algorithm itself can be described as follows:
    Bytes [0, 1] = 0x43, 0x47
    Byte  [2] = unsigned char [1-4] describing how many bytes to read for address
    The following 1-4 bytes contain the address of a table at the end of the file. 
    Following the address is the compressed data.
    After all the compressed data, sits a table with 1 or more entries
    Entry byte [0] = unsigned char [1-4] describing how many bytes to read to describe "start index"
    The next 1-4 bytes describe the address of the start index in big endian
    The next byte is an unsigned char describing how many bytes to read to describe the "count"
    The next 1-4 bytes is how many times pattern occurs as a big endian integer.
    The final byte in a table entry is the actual byte value that was repeated
    Ex: {0x02, 0x0512, 0x04, 0x08432543, 0xFF} means the byte 0xFF occured at address 0x512 0x08432543 times.

Example of before and after compression:

Before- 

![image](https://github.com/camisbored/CamsCompression/assets/81730723/6e0c4277-d05a-476a-af2d-b3b28b5305df)

After (I highlighted the pattern table)-
![image](https://github.com/camisbored/CamsCompression/assets/81730723/b5a06b34-6af3-434f-9930-aa2119a68fa3)
