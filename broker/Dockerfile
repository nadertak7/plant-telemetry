FROM eclipse-mosquitto:2.0.15

EXPOSE 1883

COPY config/mosquitto.conf /mosquitto/config/mosquitto.conf

COPY --chmod=755 entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]

# Default CMD in image - overriden by entrypoint
CMD ["mosquitto", "-c", "/mosquitto/config/mosquitto.conf"]
