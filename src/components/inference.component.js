import { useEffect, useState } from "react";
import { get_model, js_annotate_images } from "wasm-model";

const IOU_THRESHOLD = 0.50;
const CONF_THRESHOLD = 0.50;
const SHRINK_WIDTH = 32 * 7;
const SHRINK_HEIGHT = 32 * 7;
const MODEL_SIZE = "s"; // n, s, m, l, x


const InferenceWebcam = ({ frame, setAnnotatedImgSrc }) => {

    const [isLoaded, setIsLoaded] = useState(false);

    useEffect(() => {
        get_model(MODEL_SIZE).then(() => {
            console.log("Downloaded")
            setIsLoaded(true);
        })
    }, []);

    if (isLoaded) {
        console.log("is loaded?: %s", isLoaded);

        if (frame) {
            const sure_frame = frame;
            const now = performance.now();
            const annotated_img = js_annotate_images(
                sure_frame, SHRINK_WIDTH, SHRINK_HEIGHT, CONF_THRESHOLD, IOU_THRESHOLD
            );
            setAnnotatedImgSrc(annotated_img);
            const delta = performance.now() - now;
            console.log("execution time: %s", delta)
            return (
                <div>
                    <h4>Annotated Video Stream</h4>
                </div>
            );
        }

        return (<div>Setting up camera</div>)
    } else {
        return <div>Loading YOLO model...</div>;
    }
}


export default InferenceWebcam;