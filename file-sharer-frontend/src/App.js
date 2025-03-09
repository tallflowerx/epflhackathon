// src/App.js
import React, { useState } from 'react';
import { Button, Container, Typography, Paper, TextField } from '@mui/material';
import FileUploadIcon from '@mui/icons-material/FileUpload';
import axios from 'axios';

function App() {
  const [selectedFile, setSelectedFile] = useState(null);
  const [message, setMessage] = useState("");

  const uploadFile = async (event) => {
    event.preventDefault();
    if (!selectedFile) {
      setMessage("Select a file first!");
      return;
    }
    const reader = new FileReader();
    reader.onloadend = async () => {
      const base64 = reader.result.split(",")[1];
      try {
        let response = await axios.post("http://localhost:8000/upload", {
          file_name: selectedFile.name,
          content_base64: base64,
        });
        setMessage(`IPFS: ${response.data.ipfs_hash}, Public Key: ${response.data.public_key}`);
      } catch (err) {
        setMessage("Upload failed, please check backend connection!");
      }
    };
    reader.readAsDataURL(selectedFile);
  };

  return (
      <Container maxWidth="sm" style={{ marginTop: '50px' }}>
        <Paper elevation={6} style={{ padding: '20px' }}>
          <Typography variant="h4" gutterBottom align='center'>
            🚀 Blockchain File Sharer
          </Typography>
          <Typography variant="subtitle1" align='center'>
            Securely store and verify property documents.
          </Typography>
          <div style={{ marginTop: '30px' }}>
            <input
                accept="application/pdf,image/*"
                style={{ display: 'none' }}
                id="button-file"
                type="file"
                onChange={(e) => setSelectedFile(e.target.files[0])}
            />
            <label htmlFor="button-file">
              <Button
                  variant="contained"
                  component="span"
                  fullWidth
                  startIcon={<FileUploadIcon />}
                  style={{ marginBottom: '20px' }}
              >
                Choose Document
              </Button>
            </label>
            {selectedFile && <Typography>{selectedFile.name}</Typography>}
            <Button
                variant="contained"
                color="secondary"
                fullWidth
                onClick={uploadFile}
            >
              Upload Document
            </Button>
            <Typography color="primary" style={{ marginTop: '20px' }}>
              {message}
            </Typography>
          </div>
        </Paper>
      </Container>
  );
}

export default App;