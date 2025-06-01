If you have OpenSSL installed from brew (brew install openssl)

You need to point the flags to the appropriate location:
```shell
export LDFLAGS="-L/usr/local/opt/openssl/lib"
export CPPFLAGS="-I/usr/local/opt/openssl/include"
pip install psycopg2
```
Edit: For Intel/Apple Silicon+,
```shell
export LDFLAGS="-L$(brew --prefix openssl)/lib"
export CPPFLAGS="-I$(brew --prefix openssl)/include"
pip install psycopg2
```
[See](https://stackoverflow.com/questions/26288042/error-installing-psycopg2-library-not-found-for-lssl)
