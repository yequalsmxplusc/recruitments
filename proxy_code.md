# Google Drive Proxy Setup Guide

This guide explains how to set up the Google Apps Script proxy to bypass service account quota limitations.

## 1. Google Cloud Console
1.  Open the [Google Cloud Console](https://console.cloud.google.com/).
2.  Select your project.
3.  Go to **APIs & Services** -> **Library**.
4.  Search for **"Google Drive API"** and ensure it is **Enabled**.

## 2. Google Apps Script
1.  Open [script.google.com](https://script.google.com).
2.  Click **"New Project"**.
3.  In the left sidebar, click the **+** next to **Services**.
4.  Find **Drive API**, select it, and click **Add**.
5.  Paste the following code into the editor:

```javascript
function doPost(e) {
  try {
    var data = JSON.parse(e.postData.contents);
    
    // Drive API v3 settings
    var fileMetadata = {
      name: data.filename,
      parents: [data.folderId]
    };
    
    var bytes = Utilities.base64Decode(data.content);
    var blob = Utilities.newBlob(bytes, data.mimeType);
    
    // Create the file using Advanced Drive Service
    var file = Drive.Files.create(fileMetadata, blob);
    
    // Set permissions (anyone with link can view)
    Drive.Permissions.create({
      role: 'reader',
      type: 'anyone'
    }, file.id);
    
    return ContentService.createTextOutput(JSON.stringify({
      id: file.id,
      url: "https://drive.google.com/file/d/" + file.id + "/view"
    })).setMimeType(ContentService.MimeType.JSON);
    
  } catch (err) {
    return ContentService.createTextOutput(JSON.stringify({
      error: err.toString()
    })).setMimeType(ContentService.MimeType.JSON);
  }
}
```

## 3. Deployment
1.  Click **Deploy** -> **New Deployment**.
2.  Select **Web App**.
3.  Set **Execute as** to **"Me"**.
4.  Set **Who has access** to **"Anyone"**.
5.  Click **Deploy**.
6.  **Copy the Web App URL**.

## 4. Backend Configuration
1.  Open your `backend/.env` file.
2.  Add the URL you copied:
    ```env
    UPLOAD_PROXY_URL=https://script.google.com/macros/s/.../exec
    ```
3.  Restart your backend server.
