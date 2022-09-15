from pathlib import Path
from parser import parser
from scanner import Scanner
from stream import StringStream

STRING_TEST = """   double getJointMaxForce(int64_t jointHandle);
        void setJointMaxForce(int64_t objectHandle, double forceOrTorque);
        int64_t createPureShape(int64_t primitiveType, int64_t options, std::vector<double> sizes, double mass, std::optional<std::vector<int64_t>> precision = {});
        void removeObject(int64_t objectHandle);
        std::tuple<std::vector<uint8_t>, std::vector<int64_t>> getVisionSensorDepthBuffer(int64_t sensorHandle, std::optional<std::vector<int64_t>> pos = {}, std::optional<std::vector<int64_t>> size = {});
        std::tuple<std::vector<uint8_t>, std::vector<int64_t>> getVisionSensorCharImage(int64_t sensorHandle, std::optional<std::vector<int64_t>> pos = {}, std::optional<std::vector<int64_t>> size = {});
        void setVisionSensorCharImage(int64_t sensorHandle, std::vector<uint8_t> image);
        """


def main():
    header = Path('assets') / Path('remote_api_header.h')

    content = header.read_text()
    stream = StringStream(content)
    scanner = Scanner(stream)

    assigns = parser(scanner, stream)

    print(assigns)

    


if __name__ == '__main__':
    main()
