import http.server
import socketserver

class MyHttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        http.server.SimpleHTTPRequestHandler.end_headers(self)

Handler = MyHttpRequestHandler

with socketserver.TCPServer(("", 8000), Handler) as httpd:
    print("Started Server: localhost:8000")
    httpd.serve_forever()