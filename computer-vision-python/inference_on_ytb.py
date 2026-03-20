import cv2
import torch
from ultralytics import YOLO
from vidgear.gears import CamGear

stream = CamGear(
    source="https://www.youtube.com/watch?v=AdqE7mFQ7Y4",
    stream_mode=True,
    logging=True,
    **{"STREAM_RESOLUTION": "1080p"}
).start()

model = YOLO("yolov8n.pt")

if torch.cuda.is_available():
    model.to("cuda")

threshold = 0.25
frame_count = 0

while True:
    frame = stream.read()
    if frame is None:
        break

    frame_count += 1
    if frame_count % 2 != 0:
        continue

    frame = cv2.resize(frame, (1920, 1080))

    results = model(frame, imgsz=640)[0]

    for box in results.boxes:
        x1, y1, x2, y2 = map(int, box.xyxy[0])
        score = float(box.conf[0])
        class_id = int(box.cls[0])

        if score > threshold:
            cv2.rectangle(frame, (x1, y1), (x2, y2), (0,255,0), 1)
            cv2.putText(frame, results.names[class_id].upper(),
                        (x1, y1-10), cv2.FONT_HERSHEY_SIMPLEX,
                        0.5, (0,255,0), 1, cv2.LINE_AA)

    cv2.imshow("Video", frame)

    if cv2.waitKey(1) & 0xFF == ord('q'):
        break

cv2.destroyAllWindows()
stream.stop()
