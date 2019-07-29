<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT_api</name>
   <tag></tag>
   <elementGuidId>c61e1a53-4b5f-44d0-bb6c-e14839bc3107</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\n  ${requestbody}\n\n\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://${url}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'localhost:3000/posts/5'</defaultValue>
      <description></description>
      <id>88fa2945-33d8-4ac2-add4-38db3f33456a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8139f0be-98e3-490a-9726-c7f921ecaa50</id>
      <masked>false</masked>
      <name>responsebody</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a7055c39-6dd3-4a0a-aa71-4a758d3213fc</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>ae284042-d8ea-47d3-9475-868f323cdd3e</id>
      <masked>false</masked>
      <name>statuscode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b34a5d33-89e8-42f6-aca6-41f7e94fe09d</id>
      <masked>false</masked>
      <name>author</name>
   </variables>
   <variables>
      <defaultValue>' &quot;title&quot;: &quot;POST2_updated&quot;,\r\n  &quot;author&quot;: &quot;Sowmya M&quot;'</defaultValue>
      <description></description>
      <id>9a1974a7-6cb9-4f58-909a-b0f4d658b87a</id>
      <masked>false</masked>
      <name>requestbody</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8467e67b-ecdf-4aa2-bcc3-89a306d2cf85</id>
      <masked>false</masked>
      <name>request</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
