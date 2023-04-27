## A simple webserver to manipulate and redirect old urls to new urls

### 1. Problem
Everyone is moving internal infrastructure to cloud. (for e.g. internal SharePoint to SharePoint Online)
Old URLS (using old hostname) is used everywhere in our day-to-day communication.
Past email, communication tools contains these old urls.
And often we share these part email, communication messages with new members.
Since the old host is shutdown, access to these old URLs becomes inaccessible.

### 2. Solution
Create a webserver with the old hostname to accept the old URLs and redirect to the new URLs.
The old URLs may need to be translated since path and query parameters may change.

### 3. Usage
The following usage only applies if you want to do the redirecting on your PC only.
A better solution would be to host this webserver centrally so everyone can use.

#### 3.1 One-time Setup
1. Add the following entry to the host file at C:\Windows\System32\drivers\etc\hosts
````
127.1.2.3 old_host_name
````
2. Launch an elevated command prompt (Run as administrator). Execute the following command.
````
netsh interface portproxy add v4tov4 listenport=80 listenaddress=127.1.2.3 connectport=8787 connectaddress=127.0.0.1
````
#### 3.2 Usage
1. Execute uri-redirector-rust.exe
2. Click on the old URLs in your email. It should be redirected to new URLs.