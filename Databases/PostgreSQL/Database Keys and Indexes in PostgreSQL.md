## `AUTO_INCREMENT`

Often, when we create multiple tables and need to `JOIN` them together, we require an integer primary key for each row to efficiently add a reference to a row in another table as a foreign key.
```postgresql
DROP TABLE users;
CREATE TABLE users (
	id SERIAL, // Automatically increment
	name VARCHAR(128),
	email VARCHAR(128) UNIQUE, // Here UNIQUE is a logical key
	PRIMARY KEY (id) // Builds index for this field
);
```

## B-Trees
A B-tree is a tree data structure that maintains sorted data and supports searches, sequential access, insertions, and deletions in logarithmic amortized time. The B-tree is optimized for systems that read and write large blocks of data. It is commonly used in databases and file systems.
![b-trees](https://upload.wikimedia.org/wikipedia/commons/thumb/6/65/B-tree.svg/800px-B-tree.svg.png)
## Hashes
A hash function is any algorithm or subroutine that maps large data sets to smaller data sets, called keys. For example, a single integer can serve as an index to an array (cf. associative array). The values returned by a hash function are called hash values, hash codes, hash sums, checksums, or simply hashes.
Hash functions are mostly used to accelerate table lookup or data comparison tasks such as finding items in a database...
![hash function](https://upload.wikimedia.org/wikipedia/commons/thumb/5/58/Hash_table_4_1_1_0_0_1_0_LL.svg/480px-Hash_table_4_1_1_0_0_1_0_LL.svg.png)
