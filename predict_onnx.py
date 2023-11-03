import numpy as np
# Compute the prediction with onnxruntime.
import onnxruntime as rt
import timeit

X_test = np.random.rand(100, 100)

with open("data/linear.onnx", "rb") as fp:
    str_ = fp.read()

def unserialize_predict():
    sess = rt.InferenceSession(str_, providers=["CPUExecutionProvider"])
    print(dir(sess))
    print(dir(sess._model_meta))
    print(sess._model_meta.version)
    print(sess._model_meta.description)
    print(sess._model_meta.producer_name)
    input_name = sess.get_inputs()[0].name
    label_name = sess.get_outputs()[0].name
    sess.run([label_name], {input_name: X_test})[0]

unserialize_predict()