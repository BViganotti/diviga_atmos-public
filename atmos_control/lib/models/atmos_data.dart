class AtmosData {
  final double averageTemp;
  final double averageHumidity;
  final String lastReadingTime;
  final double tempOne;
  final double humidityOne;
  final double tempTwo;
  final double humidityTwo;
  final bool fridgeStatus;
  final String fridgeTurnOnTime;
  final String fridgeTurnOffTime;
  final bool humidifierStatus;
  final String humidifierTurnOnTime;
  final String humidifierTurnOffTime;
  final bool dehumidifierStatus;
  final String dehumidifierTurnOnTime;
  final String dehumidifierTurnOffTime;

  AtmosData({
    required this.averageTemp,
    required this.averageHumidity,
    required this.lastReadingTime,
    required this.tempOne,
    required this.humidityOne,
    required this.tempTwo,
    required this.humidityTwo,
    required this.fridgeStatus,
    required this.fridgeTurnOnTime,
    required this.fridgeTurnOffTime,
    required this.humidifierStatus,
    required this.humidifierTurnOnTime,
    required this.humidifierTurnOffTime,
    required this.dehumidifierStatus,
    required this.dehumidifierTurnOnTime,
    required this.dehumidifierTurnOffTime,
  });

  factory AtmosData.fromJson(Map<String, dynamic> json) {
    return AtmosData(
      averageTemp: json['average_temp']?.toDouble() ?? 0.0,
      averageHumidity: json['average_humidity']?.toDouble() ?? 0.0,
      lastReadingTime: json['last_reading_time'] ?? '',
      tempOne: json['temp_1']?.toDouble() ?? 0.0,
      humidityOne: json['humidity_1']?.toDouble() ?? 0.0,
      tempTwo: json['temp_2']?.toDouble() ?? 0.0,
      humidityTwo: json['humidity_2']?.toDouble() ?? 0.0,
      fridgeStatus: json['fridge_status'] == 'On',
      fridgeTurnOnTime: json['fridge_turn_on_datetime'] ?? '',
      fridgeTurnOffTime: json['fridge_turn_off_datetime'] ?? '',
      humidifierStatus: json['humidifier_status'] == 'On',
      humidifierTurnOnTime: json['humidifier_turn_on_datetime'] ?? '',
      humidifierTurnOffTime: json['humidifier_turn_off_datetime'] ?? '',
      dehumidifierStatus: json['dehumidifier_status'] == 'On',
      dehumidifierTurnOnTime: json['dehumidifier_turn_on_datetime'] ?? '',
      dehumidifierTurnOffTime: json['dehumidifier_turn_off_datetime'] ?? '',
    );
  }

  String formattedLastReadingTime() {
    try {
      // Remove the fractional seconds and timezone offset
      String sanitizedDate = lastReadingTime.split('.')[0];
      DateTime parsedDate = DateTime.parse(sanitizedDate);
      return "${parsedDate.toLocal()}";
    } catch (e) {
      print('Error parsing date: $e');
      return lastReadingTime; // Return the original string if parsing fails
    }
  }
}
