from websockets.sync.client import connect

with connect("ws://localhost:9001") as websocket:
    with open("session.txt", "r") as f:
        session = f.read()
        for i in session.strip().split(' '):
            websocket.send(i)
    recvd = []
    while True:
        message = websocket.recv()
        recvd.append(message)
        if message.strip() == "100":
            break
    print(' '.join(recvd))
