<h1 align="center">
  Log Secure Deserializer
</h1>

# Purpose for application
LSD can convert security log files (/var/log/secure) from human readable logs to jsons, these jsons can be used for analysising log-in attempts in Grafana or other statistics apps.



The logs generated from /var/log/secure are often filled with megabytes of useless information, this program aims to generate machine readable smaller log files.

## Functionality
LSD is able to generate jsons showing
- Invalid usernames for log-in attempts 
- invalid passwords for log-in attempts 
- invalid keys for login attempts 

## Usage
```bash
git clone https://github.com/toastxc/Log-Secure-Deserializer
cd Log-Secure-Deserializer
cargo b
sudo cp /var/log/secure log.txt
sudo chmod 777 log.txt
cargo r log.txt -p
```

Integeration with Grafana is planned but has not been worked on




![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
