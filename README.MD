## Description

Download user_data from user_data from:
https://1drv.ms/u/c/a77974c33a34d9bb/ERsh23UnYUNNolip1krLfLsBbhT3UgpLenq6rd-sCUnQnw?e=6K4O0J

## Reversing guide

To debug a release APK of the LinkedIn app and capture its traffic on a virtual Android setup, follow these steps:

### **1. Set Up a Rooted Android Emulator**

- Use an emulator image with root access (e.g., Android Studio's Standard Android image **without** Google Play).
- Use api lower than 34. Recommended: Api 31
- Start the emulator with writable system:
  ```bash
  emulator -avd YourAVDName -writable-system
  ```
- Enable root access via ADB:
  ```bash
  adb root
  ```
  ** MAKE SURE YOU ARE ROOTED IN ALL OF THESE COMMANDS**

---

### **2. Install the LinkedIn APK**

You can download it from uptown
https://linkedin.en.uptodown.com/android

```bash
adb install linkedin.apk
```

---

### **3. Configure Charles Proxy**

1. **Start Charles** on your host machine.
2. Note the proxy settings:
    - **Host IP**: `10.0.2.2`  For Android emulator.
    - **Port**: `8888`.

---

### **4. Configure Proxy on the Emulator**

Open **Internet -> AndroidWifi settings -> Edit icon top right -> Proxy Manual**.

- **Host IP**: `10.0.2.2`  For Android emulator.
- **Port**: `8888`.

---

### **5. Install Charles Certificate on the Emulator**

1. Install firefox so the cert can be downloaded.
2. Download from uptown.
3. adb install firefox.apk

4. Open the browser in the emulator and navigate to `http://chls.pro/ssl`.
5. Go to settings -> Security -> Encryption and credentials -> Install certificate -> CA certificate -> Install anyway

---

### **6. Test Basic Traffic Capture**

- Open the browser and see if charles captures any kind of traffic. HTTPS will be encrypted now

---

### **8 Add SSL in charles to inclusions**

- Go to upper panel Proxy -> SSL Proxy settings -> SSL Proxying tab -> Enable SSL Proxying, then add *.linkedin.com to
  the inclusions with no port (All ports)
- Verify that SSL is being applied by launching linkedin and checking the traffic in Charles. It should be recognized as
  SSL (Check notes in charles) but fail to decrypt.

### **9 Bypass SSL encryption**

1. The system was launched as writeable. What we need to do is make our certificate trusted.
2. The user certs are in /data/misc/user/0/cacerts-added/
3. Copy / Move them to /system/etc/security/cacerts/
4. You will probably error on "file system is read only".
5. Run mount -o rw,remount /system
6. Reboot

Network traffic should be visible in charles now.

